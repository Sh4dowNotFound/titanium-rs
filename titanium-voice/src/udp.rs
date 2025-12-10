//! UDP transport for Discord Voice.
//!
//! Handles UDP socket for voice data transport and IP discovery.

use crate::crypto::{build_rtp_header, VoiceCrypto, RTP_HEADER_SIZE};
use crate::error::VoiceError;
use crate::payload::EncryptionMode;
use byteorder::{BigEndian, ByteOrder};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{debug, info};

/// Maximum size of a voice packet.
pub const MAX_PACKET_SIZE: usize = 2048;

/// Voice UDP transport.
pub struct VoiceUdp {
    /// UDP socket.
    socket: Arc<UdpSocket>,
    /// Voice server address.
    #[allow(dead_code)]
    server_addr: SocketAddr,
    /// SSRC for this connection.
    ssrc: u32,
    /// Encryption handler.
    crypto: Option<VoiceCrypto>,
    /// Sequence number.
    sequence: u16,
    /// Timestamp.
    timestamp: u32,
    /// Scratch buffer for packet construction (avoid allocations).
    packet_buffer: Vec<u8>,
}

impl VoiceUdp {
    /// Create a new voice UDP transport.
    ///
    /// # Arguments
    /// * `server_ip` - Voice server IP address.
    /// * `server_port` - Voice server UDP port.
    /// * `ssrc` - SSRC assigned by the voice server.
    pub async fn connect(server_ip: &str, server_port: u16, ssrc: u32) -> Result<Self, VoiceError> {
        // Bind to a random local port
        let socket = UdpSocket::bind("0.0.0.0:0").await?;

        let server_addr: SocketAddr = format!("{}:{}", server_ip, server_port)
            .parse()
            .map_err(|e| VoiceError::Udp(format!("Invalid server address: {}", e)))?;

        // Connect to the server (sets default destination)
        socket.connect(server_addr).await?;

        info!(addr = %server_addr, ssrc = ssrc, "Connected to voice UDP server");

        Ok(Self {
            socket: Arc::new(socket),
            server_addr,
            ssrc,
            crypto: None,
            sequence: 0,
            timestamp: 0,
            packet_buffer: vec![0u8; MAX_PACKET_SIZE],
        })
    }

    /// Perform IP discovery.
    ///
    /// Discord's IP discovery protocol sends a 74-byte packet with the SSRC,
    /// and receives back the packet with our external IP and port filled in.
    pub async fn discover_ip(&self) -> Result<(String, u16), VoiceError> {
        // Build IP discovery packet
        // Type (2 bytes) + Length (2 bytes) + SSRC (4 bytes) + Address (64 bytes) + Port (2 bytes)
        let mut packet = [0u8; 74];

        // Type: 0x0001 = Request
        BigEndian::write_u16(&mut packet[0..2], 0x0001);
        // Length: 70 bytes (74 - 4 header bytes)
        BigEndian::write_u16(&mut packet[2..4], 70);
        // SSRC
        BigEndian::write_u32(&mut packet[4..8], self.ssrc);

        // Send discovery packet
        self.socket.send(&packet).await?;

        // Receive response
        let mut response = [0u8; 74];
        let timeout = tokio::time::Duration::from_secs(5);

        match tokio::time::timeout(timeout, self.socket.recv(&mut response)).await {
            Ok(Ok(len)) if len >= 74 => {
                // Verify type: 0x0002 = Response
                let response_type = BigEndian::read_u16(&response[0..2]);
                if response_type != 0x0002 {
                    return Err(VoiceError::IpDiscovery(format!(
                        "Invalid response type: 0x{:04x}",
                        response_type
                    )));
                }

                // Extract IP address (null-terminated string at offset 8)
                let ip_bytes = &response[8..72];
                let ip_end = ip_bytes.iter().position(|&b| b == 0).unwrap_or(64);
                let ip = String::from_utf8_lossy(&ip_bytes[..ip_end]).into_owned();

                // Extract port (last 2 bytes)
                let port = BigEndian::read_u16(&response[72..74]);

                info!(ip = %ip, port = port, "IP discovery complete");
                Ok((ip, port))
            }
            Ok(Ok(len)) => Err(VoiceError::IpDiscovery(format!(
                "Response too short: {} bytes",
                len
            ))),
            Ok(Err(e)) => Err(VoiceError::IpDiscovery(format!("Receive error: {}", e))),
            Err(_) => Err(VoiceError::Timeout("IP discovery".to_string())),
        }
    }

    /// Set the encryption key and mode.
    pub fn set_encryption(
        &mut self,
        secret_key: &[u8],
        mode: EncryptionMode,
    ) -> Result<(), VoiceError> {
        if secret_key.len() != 32 {
            return Err(VoiceError::Encryption(format!(
                "Invalid key length: expected 32, got {}",
                secret_key.len()
            )));
        }

        let mut key = [0u8; 32];
        key.copy_from_slice(secret_key);
        self.crypto = Some(VoiceCrypto::new(&key, mode));

        debug!(mode = ?mode, "Encryption configured");
        Ok(())
    }

    /// Send an audio frame.
    ///
    /// # Arguments
    /// * `opus_data` - Opus-encoded audio frame.
    ///
    /// Automatically handles RTP header generation and encryption.
    pub async fn send_audio(&mut self, opus_data: &[u8]) -> Result<(), VoiceError> {
        let crypto = self.crypto.as_mut().ok_or(VoiceError::NotConnected)?;

        // Build RTP header
        let header = build_rtp_header(self.sequence, self.timestamp, self.ssrc);

        // Increment sequence (wrapping)
        self.sequence = self.sequence.wrapping_add(1);
        // Increment timestamp by frame size (960 samples @ 48kHz for 20ms)
        self.timestamp = self.timestamp.wrapping_add(960);

        // Encrypt the audio into scratch buffer
        // Note: packet_buffer is pre-allocated. We need to ensure it's large enough.
        // MAX_PACKET_SIZE is 2048, which is enough for standard Opus frames.
        let len = crypto.encrypt_into(&header, opus_data, &mut self.packet_buffer)?;

        // Send the packet slice
        self.socket.send(&self.packet_buffer[..len]).await?;

        Ok(())
    }

    /// Receive an audio packet.
    ///
    /// Returns the decrypted audio data and the SSRC of the sender.
    pub async fn recv_audio(&self, buf: &mut [u8]) -> Result<(Vec<u8>, u32), VoiceError> {
        let crypto = self.crypto.as_ref().ok_or(VoiceError::NotConnected)?;

        // Receive packet
        let len = self.socket.recv(buf).await?;

        if len < RTP_HEADER_SIZE {
            return Err(VoiceError::Udp("Packet too short".to_string()));
        }

        // Decrypt
        let (header, audio) = crypto.decrypt(&buf[..len])?;

        // Extract SSRC from header
        let ssrc = BigEndian::read_u32(&header[8..12]);

        Ok((audio, ssrc))
    }

    /// Send a speaking state packet.
    ///
    /// This is sent periodically to keep the connection alive
    /// (5 silence frames followed by audio).
    pub async fn send_silence(&mut self) -> Result<(), VoiceError> {
        // Opus silence frame (3 bytes)
        const SILENCE_FRAME: &[u8] = &[0xF8, 0xFF, 0xFE];

        // Send 5 silence frames as per Discord protocol
        for _ in 0..5 {
            self.send_audio(SILENCE_FRAME).await?;
        }

        Ok(())
    }

    /// Get the socket's local address.
    pub fn local_addr(&self) -> Result<SocketAddr, VoiceError> {
        self.socket.local_addr().map_err(VoiceError::from)
    }

    /// Get the SSRC.
    pub fn ssrc(&self) -> u32 {
        self.ssrc
    }

    /// Get a clone of the socket for receiving.
    pub fn socket(&self) -> Arc<UdpSocket> {
        Arc::clone(&self.socket)
    }

    pub fn server_addr(&self) -> SocketAddr {
        self.server_addr
    }

    pub fn crypto(&self) -> Option<&VoiceCrypto> {
        self.crypto.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_udp_local_addr() {
        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let addr = socket.local_addr().unwrap();
        assert!(addr.port() > 0);
    }
}
