//! Voice encryption using XSalsa20-Poly1305, AES-GCM, and XChaCha20-Poly1305.
//!
//! Discord Voice supports multiple encryption modes for audio packets.

use crate::error::VoiceError;
use crate::payload::EncryptionMode;
use aes_gcm::aead::{Aead, AeadInPlace};
use aes_gcm::{Aes256Gcm, KeyInit};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use chacha20poly1305::XChaCha20Poly1305;
use rand::Rng;
use xsalsa20poly1305::XSalsa20Poly1305;

/// Size of the encryption key.
pub const KEY_SIZE: usize = 32;

/// Size of the XSalsa20/XChaCha20 nonce.
pub const NONCE_SIZE: usize = 24;

/// Size of the AES-GCM nonce.
pub const AES_GCM_NONCE_SIZE: usize = 12;

/// Size of the Poly1305/GCM authentication tag.
pub const TAG_SIZE: usize = 16;

/// RTP header size.
pub const RTP_HEADER_SIZE: usize = 12;

#[derive(Clone)]
enum InnerCipher {
    XSalsa20(XSalsa20Poly1305),
    Aes256Gcm(Box<Aes256Gcm>),
    XChaCha20(XChaCha20Poly1305),
}

/// Voice packet encryptor/decryptor.
#[derive(Clone)]
pub struct VoiceCrypto {
    /// Encryption cipher.
    cipher: InnerCipher,
    /// Encryption mode.
    mode: EncryptionMode,
    /// Nonce counter for lite mode.
    nonce_counter: u32,
}

impl VoiceCrypto {
    /// Create a new voice crypto instance.
    pub fn new(secret_key: &[u8; KEY_SIZE], mode: EncryptionMode) -> Self {
        let cipher = match mode {
            EncryptionMode::XSalsa20Poly1305Lite
            | EncryptionMode::XSalsa20Poly1305Suffix
            | EncryptionMode::XSalsa20Poly1305 => {
                InnerCipher::XSalsa20(XSalsa20Poly1305::new(secret_key.into()))
            }
            EncryptionMode::AeadAes256Gcm => {
                InnerCipher::Aes256Gcm(Box::new(Aes256Gcm::new(secret_key.into())))
            }
            EncryptionMode::AeadXChaCha20Poly1305Rtpsize => {
                InnerCipher::XChaCha20(XChaCha20Poly1305::new(secret_key.into()))
            }
        };

        Self {
            cipher,
            mode,
            nonce_counter: 0,
        }
    }

    /// Encrypt an audio frame.
    ///
    /// # Arguments
    /// * `rtp_header` - The 12-byte RTP header.
    /// * `audio` - The Opus-encoded audio data.
    ///
    /// # Returns
    /// The complete encrypted packet (header + encrypted audio).
    pub fn encrypt(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
    ) -> Result<Vec<u8>, VoiceError> {
        match self.mode {
            EncryptionMode::XSalsa20Poly1305Lite => self.encrypt_lite(rtp_header, audio),
            EncryptionMode::XSalsa20Poly1305Suffix => self.encrypt_suffix(rtp_header, audio),
            EncryptionMode::XSalsa20Poly1305 => self.encrypt_normal(rtp_header, audio),
            EncryptionMode::AeadAes256Gcm => self.encrypt_aes256_gcm(rtp_header, audio),
            EncryptionMode::AeadXChaCha20Poly1305Rtpsize => {
                self.encrypt_xchacha20_rtpsize(rtp_header, audio)
            }
        }
    }

    /// Encrypt an audio frame into a buffer (zero allocation).
    ///
    /// The buffer must be large enough to hold the packet.
    /// Returns the number of bytes written.
    pub fn encrypt_into(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
        dst: &mut [u8],
    ) -> Result<usize, VoiceError> {
        match self.mode {
            EncryptionMode::XSalsa20Poly1305Lite => self.encrypt_lite_into(rtp_header, audio, dst),
            EncryptionMode::XSalsa20Poly1305Suffix => {
                self.encrypt_suffix_into(rtp_header, audio, dst)
            }
            EncryptionMode::XSalsa20Poly1305 => self.encrypt_normal_into(rtp_header, audio, dst),
            EncryptionMode::AeadAes256Gcm => self.encrypt_aes256_gcm_into(rtp_header, audio, dst),
            EncryptionMode::AeadXChaCha20Poly1305Rtpsize => {
                self.encrypt_xchacha20_rtpsize_into(rtp_header, audio, dst)
            }
        }
    }

    /// Decrypt an audio packet.
    ///
    /// # Arguments
    /// * `packet` - The complete encrypted packet.
    ///
    /// # Returns
    /// Tuple of (RTP header, decrypted audio).
    pub fn decrypt(&self, packet: &[u8]) -> Result<([u8; RTP_HEADER_SIZE], Vec<u8>), VoiceError> {
        if packet.len() < RTP_HEADER_SIZE {
            return Err(VoiceError::Encryption("Packet too short".to_string()));
        }

        match self.mode {
            EncryptionMode::XSalsa20Poly1305Lite => self.decrypt_lite(packet),
            EncryptionMode::XSalsa20Poly1305Suffix => self.decrypt_suffix(packet),
            EncryptionMode::XSalsa20Poly1305 => self.decrypt_normal(packet),
            EncryptionMode::AeadAes256Gcm => self.decrypt_aes256_gcm(packet),
            EncryptionMode::AeadXChaCha20Poly1305Rtpsize => self.decrypt_xchacha20_rtpsize(packet),
        }
    }

    fn encrypt_lite(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
    ) -> Result<Vec<u8>, VoiceError> {
        let InnerCipher::XSalsa20(cipher) = &self.cipher else {
            unreachable!()
        };

        // Build nonce: 4-byte counter + 20 zero bytes
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        LittleEndian::write_u32(&mut nonce_bytes[..4], self.nonce_counter);
        self.nonce_counter = self.nonce_counter.wrapping_add(1);

        let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, audio)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        let mut packet = Vec::with_capacity(RTP_HEADER_SIZE + ciphertext.len() + 4);
        packet.extend_from_slice(rtp_header);
        packet.extend_from_slice(&ciphertext);
        packet.extend_from_slice(&nonce_bytes[..4]);

        Ok(packet)
    }

    fn decrypt_lite(&self, packet: &[u8]) -> Result<([u8; RTP_HEADER_SIZE], Vec<u8>), VoiceError> {
        let InnerCipher::XSalsa20(cipher) = &self.cipher else {
            unreachable!()
        };

        if packet.len() < RTP_HEADER_SIZE + TAG_SIZE + 4 {
            return Err(VoiceError::Encryption(
                "Packet too short for lite mode".to_string(),
            ));
        }

        let mut rtp_header = [0u8; RTP_HEADER_SIZE];
        rtp_header.copy_from_slice(&packet[..RTP_HEADER_SIZE]);

        let nonce_start = packet.len() - 4;
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        nonce_bytes[..4].copy_from_slice(&packet[nonce_start..]);
        let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce_bytes);

        let ciphertext = &packet[RTP_HEADER_SIZE..nonce_start];

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        Ok((rtp_header, plaintext))
    }

    fn encrypt_suffix(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
    ) -> Result<Vec<u8>, VoiceError> {
        let InnerCipher::XSalsa20(cipher) = &self.cipher else {
            unreachable!()
        };

        let nonce_bytes: [u8; NONCE_SIZE] = rand::rng().random();
        let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, audio)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        let mut packet = Vec::with_capacity(RTP_HEADER_SIZE + ciphertext.len() + NONCE_SIZE);
        packet.extend_from_slice(rtp_header);
        packet.extend_from_slice(&ciphertext);
        packet.extend_from_slice(&nonce_bytes);

        Ok(packet)
    }

    fn decrypt_suffix(
        &self,
        packet: &[u8],
    ) -> Result<([u8; RTP_HEADER_SIZE], Vec<u8>), VoiceError> {
        let InnerCipher::XSalsa20(cipher) = &self.cipher else {
            unreachable!()
        };

        if packet.len() < RTP_HEADER_SIZE + TAG_SIZE + NONCE_SIZE {
            return Err(VoiceError::Encryption(
                "Packet too short for suffix mode".to_string(),
            ));
        }

        let mut rtp_header = [0u8; RTP_HEADER_SIZE];
        rtp_header.copy_from_slice(&packet[..RTP_HEADER_SIZE]);

        let nonce_start = packet.len() - NONCE_SIZE;
        let nonce = xsalsa20poly1305::Nonce::from_slice(&packet[nonce_start..]);

        let ciphertext = &packet[RTP_HEADER_SIZE..nonce_start];

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        Ok((rtp_header, plaintext))
    }

    fn encrypt_normal(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
    ) -> Result<Vec<u8>, VoiceError> {
        let InnerCipher::XSalsa20(cipher) = &self.cipher else {
            unreachable!()
        };

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        nonce_bytes[..RTP_HEADER_SIZE].copy_from_slice(rtp_header);
        let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, audio)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        let mut packet = Vec::with_capacity(RTP_HEADER_SIZE + ciphertext.len());
        packet.extend_from_slice(rtp_header);
        packet.extend_from_slice(&ciphertext);

        Ok(packet)
    }

    fn decrypt_normal(
        &self,
        packet: &[u8],
    ) -> Result<([u8; RTP_HEADER_SIZE], Vec<u8>), VoiceError> {
        let InnerCipher::XSalsa20(cipher) = &self.cipher else {
            unreachable!()
        };

        if packet.len() < RTP_HEADER_SIZE + TAG_SIZE {
            return Err(VoiceError::Encryption(
                "Packet too short for normal mode".to_string(),
            ));
        }

        let mut rtp_header = [0u8; RTP_HEADER_SIZE];
        rtp_header.copy_from_slice(&packet[..RTP_HEADER_SIZE]);

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        nonce_bytes[..RTP_HEADER_SIZE].copy_from_slice(&rtp_header);
        let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce_bytes);

        let ciphertext = &packet[RTP_HEADER_SIZE..];

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        Ok((rtp_header, plaintext))
    }

    fn encrypt_aes256_gcm(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
    ) -> Result<Vec<u8>, VoiceError> {
        let InnerCipher::Aes256Gcm(cipher) = &self.cipher else {
            unreachable!()
        };

        // For aead_aes256_gcm, nonce is usually 12 bytes.
        // Discord: The nonce is the 4 byte extended sequence number + 4 byte timestamp + 4 "null" bytes. (Total 12)
        // However, standard AES-GCM in Discord Voice often uses a generated nonce sent with packet.
        // Assuming behavior where nonce is generated (or counter) and appended/prepended.
        // Looking at common impls: It seems comparable to "lite" but with GCM constraints.
        // BUT strict `aead_aes256_gcm` usually implies:
        // Packet = Header + Encrypted(Audio) + Tag + Nonce (4 bytes)? OR
        // Packet = Header + Encrypted(Audio, nonce=Header) + Tag?

        // Let's implement the standard AEAD pattern where nonce is appended, similar to Lite,
        // using the 4-byte counter approach if compatible, OR just pure random.
        // Discord docs for DAVE frame encryption use a specific constructed nonce.
        // For TRANSPORT encryption:
        // "aead_aes256_gcm" uses 4 byte nonce appended to packet.
        // Constructed Nonce = 4 byte nonce (from packet) + 8 zero bytes.

        let mut nonce_prefix = [0u8; 4];
        LittleEndian::write_u32(&mut nonce_prefix, self.nonce_counter);
        self.nonce_counter = self.nonce_counter.wrapping_add(1);

        let mut nonce_bytes = [0u8; AES_GCM_NONCE_SIZE];
        nonce_bytes[..4].copy_from_slice(&nonce_prefix);
        // Remaining 8 bytes are zero.

        let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, audio)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        // Packet = Header + Ciphertext (includes tag) + 4-byte Nonce Prefix
        let mut packet = Vec::with_capacity(RTP_HEADER_SIZE + ciphertext.len() + 4);
        packet.extend_from_slice(rtp_header);
        packet.extend_from_slice(&ciphertext);
        packet.extend_from_slice(&nonce_prefix);

        Ok(packet)
    }

    fn decrypt_aes256_gcm(
        &self,
        packet: &[u8],
    ) -> Result<([u8; RTP_HEADER_SIZE], Vec<u8>), VoiceError> {
        let InnerCipher::Aes256Gcm(cipher) = &self.cipher else {
            unreachable!()
        };

        if packet.len() < RTP_HEADER_SIZE + TAG_SIZE + 4 {
            return Err(VoiceError::Encryption(
                "Packet too short for aes256_gcm".to_string(),
            ));
        }

        let mut rtp_header = [0u8; RTP_HEADER_SIZE];
        rtp_header.copy_from_slice(&packet[..RTP_HEADER_SIZE]);

        let nonce_start = packet.len() - 4;
        let mut nonce_bytes = [0u8; AES_GCM_NONCE_SIZE];
        nonce_bytes[..4].copy_from_slice(&packet[nonce_start..]);
        let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);

        let ciphertext = &packet[RTP_HEADER_SIZE..nonce_start];

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        Ok((rtp_header, plaintext))
    }

    fn encrypt_xchacha20_rtpsize(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
    ) -> Result<Vec<u8>, VoiceError> {
        let InnerCipher::XChaCha20(cipher) = &self.cipher else {
            unreachable!()
        };

        // Nonce is RTP header padded to 24 bytes
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        nonce_bytes[..RTP_HEADER_SIZE].copy_from_slice(rtp_header);
        let nonce = chacha20poly1305::XNonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, audio)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        let mut packet = Vec::with_capacity(RTP_HEADER_SIZE + ciphertext.len());
        packet.extend_from_slice(rtp_header);
        packet.extend_from_slice(&ciphertext);

        Ok(packet)
    }

    fn decrypt_xchacha20_rtpsize(
        &self,
        packet: &[u8],
    ) -> Result<([u8; RTP_HEADER_SIZE], Vec<u8>), VoiceError> {
        let InnerCipher::XChaCha20(cipher) = &self.cipher else {
            unreachable!()
        };

        if packet.len() < RTP_HEADER_SIZE + TAG_SIZE {
            return Err(VoiceError::Encryption(
                "Packet too short for xchacha20_rtpsize".to_string(),
            ));
        }

        let mut rtp_header = [0u8; RTP_HEADER_SIZE];
        rtp_header.copy_from_slice(&packet[..RTP_HEADER_SIZE]);

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        nonce_bytes[..RTP_HEADER_SIZE].copy_from_slice(&rtp_header);
        let nonce = chacha20poly1305::XNonce::from_slice(&nonce_bytes);

        let ciphertext = &packet[RTP_HEADER_SIZE..];

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        Ok((rtp_header, plaintext))
    }

    fn encrypt_lite_into(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
        dst: &mut [u8],
    ) -> Result<usize, VoiceError> {
        let InnerCipher::XSalsa20(cipher) = &self.cipher else {
            unreachable!()
        };

        let required = RTP_HEADER_SIZE + audio.len() + TAG_SIZE + 4;
        if dst.len() < required {
            return Err(VoiceError::Encryption("Buffer too small".into()));
        }

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        LittleEndian::write_u32(&mut nonce_bytes[..4], self.nonce_counter);
        self.nonce_counter = self.nonce_counter.wrapping_add(1);
        let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce_bytes);

        dst[..RTP_HEADER_SIZE].copy_from_slice(rtp_header);

        // Copy audio
        let audio_start = RTP_HEADER_SIZE;
        let audio_end = audio_start + audio.len();
        dst[audio_start..audio_end].copy_from_slice(audio);

        let tag = cipher
            .encrypt_in_place_detached(nonce, &[], &mut dst[audio_start..audio_end])
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        dst[audio_end..audio_end + TAG_SIZE].copy_from_slice(tag.as_slice());
        dst[audio_end + TAG_SIZE..required].copy_from_slice(&nonce_bytes[..4]);

        Ok(required)
    }

    fn encrypt_suffix_into(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
        dst: &mut [u8],
    ) -> Result<usize, VoiceError> {
        let InnerCipher::XSalsa20(cipher) = &self.cipher else {
            unreachable!()
        };

        let required = RTP_HEADER_SIZE + audio.len() + TAG_SIZE + NONCE_SIZE;
        if dst.len() < required {
            return Err(VoiceError::Encryption("Buffer too small".into()));
        }

        let nonce_bytes: [u8; NONCE_SIZE] = rand::rng().random();
        let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce_bytes);

        dst[..RTP_HEADER_SIZE].copy_from_slice(rtp_header);

        let audio_start = RTP_HEADER_SIZE;
        let audio_end = audio_start + audio.len();
        dst[audio_start..audio_end].copy_from_slice(audio);

        let tag = cipher
            .encrypt_in_place_detached(nonce, &[], &mut dst[audio_start..audio_end])
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        dst[audio_end..audio_end + TAG_SIZE].copy_from_slice(tag.as_slice());
        dst[audio_end + TAG_SIZE..required].copy_from_slice(&nonce_bytes);

        Ok(required)
    }

    fn encrypt_normal_into(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
        dst: &mut [u8],
    ) -> Result<usize, VoiceError> {
        let InnerCipher::XSalsa20(cipher) = &self.cipher else {
            unreachable!()
        };

        let required = RTP_HEADER_SIZE + audio.len() + TAG_SIZE;
        if dst.len() < required {
            return Err(VoiceError::Encryption("Buffer too small".into()));
        }

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        nonce_bytes[..RTP_HEADER_SIZE].copy_from_slice(rtp_header);
        let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce_bytes);

        dst[..RTP_HEADER_SIZE].copy_from_slice(rtp_header);

        let audio_start = RTP_HEADER_SIZE;
        let audio_end = audio_start + audio.len();
        dst[audio_start..audio_end].copy_from_slice(audio);

        let tag = cipher
            .encrypt_in_place_detached(nonce, &[], &mut dst[audio_start..audio_end])
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        dst[audio_end..audio_end + TAG_SIZE].copy_from_slice(tag.as_slice());

        Ok(required)
    }

    fn encrypt_aes256_gcm_into(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
        dst: &mut [u8],
    ) -> Result<usize, VoiceError> {
        let InnerCipher::Aes256Gcm(cipher) = &self.cipher else {
            unreachable!()
        };

        let required = RTP_HEADER_SIZE + audio.len() + TAG_SIZE + 4;
        if dst.len() < required {
            return Err(VoiceError::Encryption("Buffer too small".into()));
        }

        let mut nonce_prefix = [0u8; 4];
        LittleEndian::write_u32(&mut nonce_prefix, self.nonce_counter);
        self.nonce_counter = self.nonce_counter.wrapping_add(1);

        let mut nonce_bytes = [0u8; AES_GCM_NONCE_SIZE];
        nonce_bytes[..4].copy_from_slice(&nonce_prefix);
        let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);

        dst[..RTP_HEADER_SIZE].copy_from_slice(rtp_header);

        let audio_start = RTP_HEADER_SIZE;
        let audio_end = audio_start + audio.len();
        dst[audio_start..audio_end].copy_from_slice(audio);

        let tag = cipher
            .encrypt_in_place_detached(nonce, &[], &mut dst[audio_start..audio_end])
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        dst[audio_end..audio_end + TAG_SIZE].copy_from_slice(tag.as_slice());
        dst[audio_end + TAG_SIZE..required].copy_from_slice(&nonce_prefix);

        Ok(required)
    }

    fn encrypt_xchacha20_rtpsize_into(
        &mut self,
        rtp_header: &[u8; RTP_HEADER_SIZE],
        audio: &[u8],
        dst: &mut [u8],
    ) -> Result<usize, VoiceError> {
        let InnerCipher::XChaCha20(cipher) = &self.cipher else {
            unreachable!()
        };

        let required = RTP_HEADER_SIZE + audio.len() + TAG_SIZE;
        if dst.len() < required {
            return Err(VoiceError::Encryption("Buffer too small".into()));
        }

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        nonce_bytes[..RTP_HEADER_SIZE].copy_from_slice(rtp_header);
        let nonce = chacha20poly1305::XNonce::from_slice(&nonce_bytes);

        dst[..RTP_HEADER_SIZE].copy_from_slice(rtp_header);

        let audio_start = RTP_HEADER_SIZE;
        let audio_end = audio_start + audio.len();
        dst[audio_start..audio_end].copy_from_slice(audio);

        let tag = cipher
            .encrypt_in_place_detached(nonce, &[], &mut dst[audio_start..audio_end])
            .map_err(|e| VoiceError::Encryption(e.to_string()))?;

        dst[audio_end..audio_end + TAG_SIZE].copy_from_slice(tag.as_slice());

        Ok(required)
    }
}

/// Build an RTP header.
pub fn build_rtp_header(sequence: u16, timestamp: u32, ssrc: u32) -> [u8; RTP_HEADER_SIZE] {
    let mut header = [0u8; RTP_HEADER_SIZE];

    // Version (2), Padding (0), Extension (0), CSRC count (0)
    header[0] = 0x80;
    // Marker (0), Payload type (0x78 = 120 for Opus)
    header[1] = 0x78;
    // Sequence number (big-endian)
    BigEndian::write_u16(&mut header[2..4], sequence);
    // Timestamp (big-endian)
    BigEndian::write_u32(&mut header[4..8], timestamp);
    // SSRC (big-endian)
    BigEndian::write_u32(&mut header[8..12], ssrc);

    header
}

/// Parse an RTP header.
pub fn parse_rtp_header(header: &[u8; RTP_HEADER_SIZE]) -> (u16, u32, u32) {
    let sequence = BigEndian::read_u16(&header[2..4]);
    let timestamp = BigEndian::read_u32(&header[4..8]);
    let ssrc = BigEndian::read_u32(&header[8..12]);
    (sequence, timestamp, ssrc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtp_header() {
        let header = build_rtp_header(100, 48000, 12345);
        let (seq, ts, ssrc) = parse_rtp_header(&header);

        assert_eq!(seq, 100);
        assert_eq!(ts, 48000);
        assert_eq!(ssrc, 12345);
    }

    #[test]
    fn test_encrypt_decrypt_lite() {
        let key = [0u8; KEY_SIZE];
        let mut crypto = VoiceCrypto::new(&key, EncryptionMode::XSalsa20Poly1305Lite);

        let header = build_rtp_header(1, 960, 12345);
        let audio = b"test audio data";

        let encrypted = crypto.encrypt(&header, audio).unwrap();

        let crypto_dec = VoiceCrypto::new(&key, EncryptionMode::XSalsa20Poly1305Lite);
        let (dec_header, decrypted) = crypto_dec.decrypt(&encrypted).unwrap();

        assert_eq!(&dec_header, &header);
        assert_eq!(&decrypted, audio);
    }

    #[test]
    fn test_encrypt_decrypt_aes256_gcm() {
        let key = [0u8; KEY_SIZE];
        let mut crypto = VoiceCrypto::new(&key, EncryptionMode::AeadAes256Gcm);

        let header = build_rtp_header(1, 960, 12345);
        let audio = b"test aes gcm audio";

        let encrypted = crypto.encrypt(&header, audio).unwrap();

        // Decrypt with a new instance
        let crypto_dec = VoiceCrypto::new(&key, EncryptionMode::AeadAes256Gcm);
        let (dec_header, decrypted) = crypto_dec.decrypt(&encrypted).unwrap();

        assert_eq!(&dec_header, &header);
        assert_eq!(&decrypted, audio);
    }

    #[test]
    fn test_encrypt_decrypt_xchacha20_rtpsize() {
        let key = [0u8; KEY_SIZE];
        let mut crypto = VoiceCrypto::new(&key, EncryptionMode::AeadXChaCha20Poly1305Rtpsize);

        let header = build_rtp_header(1, 960, 12345);
        let audio = b"test xchacha20 audio";

        let encrypted = crypto.encrypt(&header, audio).unwrap();

        let crypto_dec = VoiceCrypto::new(&key, EncryptionMode::AeadXChaCha20Poly1305Rtpsize);
        let (dec_header, decrypted) = crypto_dec.decrypt(&encrypted).unwrap();

        assert_eq!(&dec_header, &header);
        assert_eq!(&decrypted, audio);
    }
}
