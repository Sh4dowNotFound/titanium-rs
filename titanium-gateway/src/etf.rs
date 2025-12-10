//! Erlang Term Format (ETF) decoder for Discord Gateway.
//!
//! Discord supports ETF as an alternative to JSON for Gateway payloads.
//! ETF is more compact (~30% smaller) and can be faster to parse.
//!
//! # ETF Format
//!
//! ETF is Erlang's external binary format. Discord uses a subset:
//! - Atoms (as strings)
//! - Integers (small, big)
//! - Floats
//! - Binaries (as strings)
//! - Lists
//! - Maps
//! - Nil (empty list)
//!
//! # Usage
//!
//! ```ignore
//! use titanium_gateway::etf::EtfDecoder;
//!
//! let bytes: &[u8] = /* ETF-encoded payload */;
//! let value = EtfDecoder::decode(bytes)?;
//! let json = EtfDecoder::to_json_value(&value)?;
//! ```

use crate::error::GatewayError;

/// ETF format version tag.
const ETF_VERSION: u8 = 131;

/// ETF term tags.
mod tags {
    pub const SMALL_INTEGER: u8 = 97;
    pub const INTEGER: u8 = 98;
    pub const FLOAT: u8 = 99;
    pub const ATOM: u8 = 100;
    pub const SMALL_TUPLE: u8 = 104;
    pub const LARGE_TUPLE: u8 = 105;
    pub const NIL: u8 = 106;
    pub const STRING: u8 = 107;
    pub const LIST: u8 = 108;
    pub const BINARY: u8 = 109;
    pub const SMALL_BIG: u8 = 110;
    pub const LARGE_BIG: u8 = 111;
    pub const MAP: u8 = 116;
    pub const ATOM_UTF8: u8 = 118;
    pub const SMALL_ATOM_UTF8: u8 = 119;
    pub const NEW_FLOAT: u8 = 70;
    pub const COMPRESSED: u8 = 80;
}

/// An ETF term (Erlang value).
#[derive(Debug, Clone, PartialEq)]
pub enum EtfTerm {
    /// A small integer (0-255).
    SmallInt(u8),
    /// A signed 32-bit integer.
    Int(i32),
    /// A big integer (arbitrary precision).
    BigInt(i128),
    /// A 64-bit floating point number.
    Float(f64),
    /// An atom (interned string in Erlang, just a string for us).
    Atom(String),
    /// A tuple (fixed-size array).
    Tuple(Vec<EtfTerm>),
    /// Nil (empty list, often used as "null").
    Nil,
    /// A string (list of small integers in ETF).
    String(String),
    /// A list of terms.
    List(Vec<EtfTerm>),
    /// A binary (raw bytes, often UTF-8 strings in Discord's usage).
    Binary(Vec<u8>),
    /// A map (key-value pairs).
    Map(Vec<(EtfTerm, EtfTerm)>),
}

/// ETF decoder.
pub struct EtfDecoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> EtfDecoder<'a> {
    /// Create a new decoder for the given ETF bytes.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    /// Decode an ETF payload into an EtfTerm.
    pub fn decode(data: &[u8]) -> Result<EtfTerm, GatewayError> {
        let mut decoder = EtfDecoder::new(data);
        decoder.decode_term()
    }

    /// Decode the entire payload.
    fn decode_term(&mut self) -> Result<EtfTerm, GatewayError> {
        // Check version byte
        let version = self.read_u8()?;
        if version != ETF_VERSION {
            return Err(GatewayError::JsonDecode(format!(
                "Invalid ETF version: expected {}, got {}",
                ETF_VERSION, version
            )));
        }

        self.decode_value()
    }

    /// Decode a single value.
    fn decode_value(&mut self) -> Result<EtfTerm, GatewayError> {
        let tag = self.read_u8()?;

        match tag {
            tags::SMALL_INTEGER => {
                let value = self.read_u8()?;
                Ok(EtfTerm::SmallInt(value))
            }

            tags::INTEGER => {
                let value = self.read_i32()?;
                Ok(EtfTerm::Int(value))
            }

            tags::FLOAT => {
                // Old float format: 31 bytes ASCII representation
                let bytes = self.read_bytes(31)?;
                let s = std::str::from_utf8(bytes)
                    .map_err(|e| GatewayError::JsonDecode(format!("Invalid float string: {}", e)))?
                    .trim_end_matches('\0');
                let value: f64 = s
                    .parse()
                    .map_err(|e| GatewayError::JsonDecode(format!("Invalid float: {}", e)))?;
                Ok(EtfTerm::Float(value))
            }

            tags::NEW_FLOAT => {
                let bytes = self.read_bytes(8)?;
                let value =
                    f64::from_be_bytes(bytes.try_into().map_err(|_| {
                        GatewayError::JsonDecode("Invalid float bytes".to_string())
                    })?);
                Ok(EtfTerm::Float(value))
            }

            tags::ATOM => {
                let len = self.read_u16()? as usize;
                let bytes = self.read_bytes(len)?;
                let s = String::from_utf8_lossy(bytes).into_owned();
                Ok(EtfTerm::Atom(s))
            }

            tags::ATOM_UTF8 => {
                let len = self.read_u16()? as usize;
                let bytes = self.read_bytes(len)?;
                let s = String::from_utf8_lossy(bytes).into_owned();
                Ok(EtfTerm::Atom(s))
            }

            tags::SMALL_ATOM_UTF8 => {
                let len = self.read_u8()? as usize;
                let bytes = self.read_bytes(len)?;
                let s = String::from_utf8_lossy(bytes).into_owned();
                Ok(EtfTerm::Atom(s))
            }

            tags::SMALL_TUPLE => {
                let arity = self.read_u8()? as usize;
                let mut elements = Vec::with_capacity(arity);
                for _ in 0..arity {
                    elements.push(self.decode_value()?);
                }
                Ok(EtfTerm::Tuple(elements))
            }

            tags::LARGE_TUPLE => {
                let arity = self.read_u32()? as usize;
                let mut elements = Vec::with_capacity(arity);
                for _ in 0..arity {
                    elements.push(self.decode_value()?);
                }
                Ok(EtfTerm::Tuple(elements))
            }

            tags::NIL => Ok(EtfTerm::Nil),

            tags::STRING => {
                // List of small integers (0-255)
                let len = self.read_u16()? as usize;
                let bytes = self.read_bytes(len)?;
                let s = String::from_utf8_lossy(bytes).into_owned();
                Ok(EtfTerm::String(s))
            }

            tags::LIST => {
                let len = self.read_u32()? as usize;
                let mut elements = Vec::with_capacity(len);
                for _ in 0..len {
                    elements.push(self.decode_value()?);
                }
                // Lists in ETF have a tail (usually nil)
                let _tail = self.decode_value()?;
                Ok(EtfTerm::List(elements))
            }

            tags::BINARY => {
                let len = self.read_u32()? as usize;
                let bytes = self.read_bytes(len)?;
                Ok(EtfTerm::Binary(bytes.to_vec()))
            }

            tags::SMALL_BIG => {
                let n = self.read_u8()? as usize;
                let sign = self.read_u8()?;
                let bytes = self.read_bytes(n)?;

                let mut value: i128 = 0;
                for (i, &byte) in bytes.iter().enumerate() {
                    value |= (byte as i128) << (i * 8);
                }

                if sign != 0 {
                    value = -value;
                }

                Ok(EtfTerm::BigInt(value))
            }

            tags::LARGE_BIG => {
                let n = self.read_u32()? as usize;
                let sign = self.read_u8()?;
                let bytes = self.read_bytes(n)?;

                // For very large integers, we'll truncate to i128
                let mut value: i128 = 0;
                for (i, &byte) in bytes.iter().take(16).enumerate() {
                    value |= (byte as i128) << (i * 8);
                }

                if sign != 0 {
                    value = -value;
                }

                Ok(EtfTerm::BigInt(value))
            }

            tags::MAP => {
                let arity = self.read_u32()? as usize;
                let mut pairs = Vec::with_capacity(arity);
                for _ in 0..arity {
                    let key = self.decode_value()?;
                    let value = self.decode_value()?;
                    pairs.push((key, value));
                }
                Ok(EtfTerm::Map(pairs))
            }

            tags::COMPRESSED => {
                let uncompressed_size = self.read_u32()? as usize;
                let compressed_data = &self.data[self.pos..];

                // Decompress using zlib
                use flate2::read::ZlibDecoder;
                use std::io::Read;

                let mut decoder = ZlibDecoder::new(compressed_data);
                let mut decompressed = Vec::with_capacity(uncompressed_size);
                decoder.read_to_end(&mut decompressed).map_err(|e| {
                    GatewayError::JsonDecode(format!("ETF decompression failed: {}", e))
                })?;

                // Skip to end of compressed data
                self.pos = self.data.len();

                // Decode the decompressed term
                let mut inner = EtfDecoder::new(&decompressed);
                inner.decode_value()
            }

            _ => Err(GatewayError::JsonDecode(format!(
                "Unknown ETF tag: {} at position {}",
                tag,
                self.pos - 1
            ))),
        }
    }

    /// Read a single byte.
    #[inline]
    fn read_u8(&mut self) -> Result<u8, GatewayError> {
        if self.pos >= self.data.len() {
            return Err(GatewayError::JsonDecode(
                "Unexpected end of ETF data".to_string(),
            ));
        }
        let byte = self.data[self.pos];
        self.pos += 1;
        Ok(byte)
    }

    /// Read a big-endian u16.
    #[inline]
    fn read_u16(&mut self) -> Result<u16, GatewayError> {
        if self.pos + 2 > self.data.len() {
            return Err(GatewayError::JsonDecode(
                "Unexpected end of ETF data".to_string(),
            ));
        }
        let value = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        Ok(value)
    }

    /// Read a big-endian u32.
    #[inline]
    fn read_u32(&mut self) -> Result<u32, GatewayError> {
        if self.pos + 4 > self.data.len() {
            return Err(GatewayError::JsonDecode(
                "Unexpected end of ETF data".to_string(),
            ));
        }
        let value = u32::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    /// Read a big-endian i32.
    #[inline]
    fn read_i32(&mut self) -> Result<i32, GatewayError> {
        if self.pos + 4 > self.data.len() {
            return Err(GatewayError::JsonDecode(
                "Unexpected end of ETF data".to_string(),
            ));
        }
        let value = i32::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        Ok(value)
    }

    /// Read n bytes.
    fn read_bytes(&mut self, n: usize) -> Result<&'a [u8], GatewayError> {
        if self.pos + n > self.data.len() {
            return Err(GatewayError::JsonDecode(
                "Unexpected end of ETF data".to_string(),
            ));
        }
        let bytes = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(bytes)
    }

    /// Convert an EtfTerm to a serde_json::Value.
    ///
    /// This allows ETF payloads to be processed using the same JSON-based
    /// event parsing logic.
    pub fn to_json_value(term: &EtfTerm) -> Result<serde_json::Value, GatewayError> {
        match term {
            EtfTerm::SmallInt(n) => Ok(serde_json::Value::Number((*n as i64).into())),
            EtfTerm::Int(n) => Ok(serde_json::Value::Number((*n as i64).into())),
            EtfTerm::BigInt(n) => {
                // For Snowflakes, Discord sends them as big integers
                // We convert to string to preserve precision
                if *n > i64::MAX as i128 || *n < i64::MIN as i128 {
                    Ok(serde_json::Value::String(n.to_string()))
                } else {
                    Ok(serde_json::Value::Number((*n as i64).into()))
                }
            }
            EtfTerm::Float(f) => serde_json::Number::from_f64(*f)
                .map(serde_json::Value::Number)
                .ok_or_else(|| GatewayError::JsonDecode("Invalid float value".to_string())),
            EtfTerm::Atom(s) => {
                // Discord uses atoms for special values
                match s.as_str() {
                    "nil" | "null" => Ok(serde_json::Value::Null),
                    "true" => Ok(serde_json::Value::Bool(true)),
                    "false" => Ok(serde_json::Value::Bool(false)),
                    _ => Ok(serde_json::Value::String(s.clone())),
                }
            }
            EtfTerm::Tuple(elements) => {
                let arr: Result<Vec<_>, _> = elements.iter().map(Self::to_json_value).collect();
                Ok(serde_json::Value::Array(arr?))
            }
            EtfTerm::Nil => Ok(serde_json::Value::Null),
            EtfTerm::String(s) => Ok(serde_json::Value::String(s.clone())),
            EtfTerm::List(elements) => {
                let arr: Result<Vec<_>, _> = elements.iter().map(Self::to_json_value).collect();
                Ok(serde_json::Value::Array(arr?))
            }
            EtfTerm::Binary(bytes) => {
                // Try to parse as UTF-8 string
                match String::from_utf8(bytes.clone()) {
                    Ok(s) => Ok(serde_json::Value::String(s)),
                    Err(_) => {
                        // Fall back to base64 encoding
                        use base64::Engine;
                        let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
                        Ok(serde_json::Value::String(encoded))
                    }
                }
            }
            EtfTerm::Map(pairs) => {
                let mut map = serde_json::Map::new();
                for (key, value) in pairs {
                    let key_str = match key {
                        EtfTerm::Atom(s) => s.clone(),
                        EtfTerm::Binary(b) => String::from_utf8_lossy(b).into_owned(),
                        EtfTerm::String(s) => s.clone(),
                        _ => {
                            // Convert non-string keys to string representation
                            let json_key = Self::to_json_value(key)?;
                            json_key.to_string()
                        }
                    };
                    map.insert(key_str, Self::to_json_value(value)?);
                }
                Ok(serde_json::Value::Object(map))
            }
        }
    }

    /// Convert an EtfTerm to a JSON string.
    pub fn to_json_string(term: &EtfTerm) -> Result<String, GatewayError> {
        let value = Self::to_json_value(term)?;
        serde_json::to_string(&value).map_err(GatewayError::from)
    }
}

/// Gateway encoding type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GatewayEncoding {
    /// JSON encoding (default, human-readable).
    #[default]
    Json,
    /// ETF encoding (smaller, faster).
    Etf,
}

impl GatewayEncoding {
    /// Get the encoding name for the Gateway URL query parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            GatewayEncoding::Json => "json",
            GatewayEncoding::Etf => "etf",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_small_int() {
        // Version byte + small_integer tag + value
        let data = [131, 97, 42];
        let term = EtfDecoder::decode(&data).unwrap();
        assert_eq!(term, EtfTerm::SmallInt(42));
    }

    #[test]
    fn test_decode_integer() {
        // Version byte + integer tag + 4 bytes BE
        let data = [131, 98, 0, 0, 1, 0]; // 256
        let term = EtfDecoder::decode(&data).unwrap();
        assert_eq!(term, EtfTerm::Int(256));
    }

    #[test]
    fn test_decode_nil() {
        let data = [131, 106];
        let term = EtfDecoder::decode(&data).unwrap();
        assert_eq!(term, EtfTerm::Nil);
    }

    #[test]
    fn test_decode_binary() {
        // Version + binary tag + length(4 bytes) + "hello"
        let data = [131, 109, 0, 0, 0, 5, b'h', b'e', b'l', b'l', b'o'];
        let term = EtfDecoder::decode(&data).unwrap();
        assert_eq!(term, EtfTerm::Binary(b"hello".to_vec()));
    }

    #[test]
    fn test_decode_small_atom_utf8() {
        // Version + small_atom_utf8 tag + length(1 byte) + "test"
        let data = [131, 119, 4, b't', b'e', b's', b't'];
        let term = EtfDecoder::decode(&data).unwrap();
        assert_eq!(term, EtfTerm::Atom("test".to_string()));
    }

    #[test]
    fn test_decode_map() {
        // Simple map: %{a: 1}
        // Version + map tag + arity(4 bytes) + key + value
        let data = [
            131, // version
            116, // map tag
            0, 0, 0, 1, // arity = 1
            119, 1, b'a', // small_atom_utf8 "a"
            97, 1, // small_integer 1
        ];
        let term = EtfDecoder::decode(&data).unwrap();

        if let EtfTerm::Map(pairs) = term {
            assert_eq!(pairs.len(), 1);
            assert_eq!(pairs[0].0, EtfTerm::Atom("a".to_string()));
            assert_eq!(pairs[0].1, EtfTerm::SmallInt(1));
        } else {
            panic!("Expected Map");
        }
    }

    #[test]
    fn test_to_json_value() {
        let term = EtfTerm::Map(vec![
            (EtfTerm::Atom("op".to_string()), EtfTerm::SmallInt(10)),
            (
                EtfTerm::Atom("d".to_string()),
                EtfTerm::Map(vec![(
                    EtfTerm::Atom("heartbeat_interval".to_string()),
                    EtfTerm::Int(41250),
                )]),
            ),
        ]);

        let json = EtfDecoder::to_json_value(&term).unwrap();
        assert_eq!(json["op"], 10);
        assert_eq!(json["d"]["heartbeat_interval"], 41250);
    }

    #[test]
    fn test_atom_special_values() {
        // nil -> null
        let term = EtfTerm::Atom("nil".to_string());
        let json = EtfDecoder::to_json_value(&term).unwrap();
        assert!(json.is_null());

        // true -> true
        let term = EtfTerm::Atom("true".to_string());
        let json = EtfDecoder::to_json_value(&term).unwrap();
        assert_eq!(json, serde_json::Value::Bool(true));

        // false -> false
        let term = EtfTerm::Atom("false".to_string());
        let json = EtfDecoder::to_json_value(&term).unwrap();
        assert_eq!(json, serde_json::Value::Bool(false));
    }
}
