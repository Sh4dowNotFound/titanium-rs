//! JSON parsing utilities with optional SIMD acceleration.
//!
//! This module provides a unified interface for JSON parsing that can use
//! either `serde_json` (default) or `simd-json` (with the `simd` feature).
//!
//! # SIMD-JSON
//!
//! When the `simd` feature is enabled, JSON parsing uses SIMD instructions
//! (SSE4.2/AVX2 on x86, NEON on ARM) for ~2-3x faster parsing.
//!
//! **Note**: SIMD-JSON requires mutable access to the input string because
//! it performs in-place parsing for zero-copy string extraction.
#![allow(unsafe_code)]

use crate::error::GatewayError;
use serde::de::DeserializeOwned;

/// Parse JSON from a string slice.
///
/// Uses `serde_json` by default, or `simd-json` when the `simd` feature is enabled.
///
/// # Arguments
/// * `json` - The JSON string to parse. With SIMD enabled, this **will be mutated** in-place.
///
/// # Errors
/// Returns `GatewayError::JsonDecode` if parsing fails.
///
/// # Safety (SIMD)
/// When `simd` feature is enabled, this function uses `unsafe` because `simd-json` modifies
/// the input string (replaces characters with nulls). The caller must ensure that:
/// 1. The string is not used afterwards if the parsed type borrows from it (zero-copy).
/// 2. If the string IS used afterwards (e.g. for logging), it will be modified/corrupted.
#[cfg(feature = "simd")]
pub fn from_str<T: DeserializeOwned>(json: &mut str) -> Result<T, GatewayError> {
    // SAFETY: simd-json requires mutable access for in-place parsing.
    // We are explicit in the signature that we take `&mut str`.
    unsafe { simd_json::from_str(json).map_err(|e| GatewayError::JsonDecode(e.to_string())) }
}

/// Parse JSON from a string slice.
///
/// Uses `serde_json` by default.
#[cfg(not(feature = "simd"))]
pub fn from_str<T: DeserializeOwned>(json: &str) -> Result<T, GatewayError> {
    serde_json::from_str(json).map_err(GatewayError::from)
}

/// Parse JSON from owned String, consuming it.
///
/// This is the preferred method when you have an owned String, as it allows
/// SIMD-JSON to perform in-place parsing without additional allocations.
#[cfg(feature = "simd")]
pub fn from_string<T: DeserializeOwned>(mut json: String) -> Result<T, GatewayError> {
    // SAFETY: We own the string and won't use it after parsing.
    // The strict ownership transfer prevents use-after-modification bugs.
    unsafe {
        simd_json::from_str(json.as_mut_str()).map_err(|e| GatewayError::JsonDecode(e.to_string()))
    }
}

/// Parse JSON from owned String.
#[cfg(not(feature = "simd"))]
pub fn from_string<T: DeserializeOwned>(json: String) -> Result<T, GatewayError> {
    serde_json::from_str(&json).map_err(GatewayError::from)
}

/// Parse JSON from a byte slice.
///
/// Useful for binary/compressed data that's been decompressed to bytes.
#[cfg(feature = "simd")]
#[allow(dead_code)]
pub fn from_slice<T: DeserializeOwned>(json: &[u8]) -> Result<T, GatewayError> {
    // simd-json requires mutable access, so we need to copy bytes to Vec
    let mut bytes = json.to_vec();
    simd_json::from_slice(&mut bytes).map_err(|e| GatewayError::JsonDecode(e.to_string()))
}

/// Parse JSON from a byte slice.
#[cfg(not(feature = "simd"))]
#[allow(dead_code)]
pub fn from_slice<T: DeserializeOwned>(json: &[u8]) -> Result<T, GatewayError> {
    serde_json::from_slice(json).map_err(GatewayError::from)
}

/// Serialize a value to a JSON string.
///
/// Always uses `serde_json` for serialization (SIMD is for parsing only).
pub fn to_string<T: serde::Serialize>(value: &T) -> Result<String, GatewayError> {
    serde_json::to_string(value).map_err(GatewayError::from)
}

/// Serialize a value to a pretty-printed JSON string.
#[allow(dead_code)]
pub fn to_string_pretty<T: serde::Serialize>(value: &T) -> Result<String, GatewayError> {
    serde_json::to_string_pretty(value).map_err(GatewayError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestPayload {
        op: u8,
        d: Option<TestData>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestData {
        heartbeat_interval: u64,
    }

    #[test]
    fn test_parse_hello_payload() {
        let json = r#"{"op":10,"d":{"heartbeat_interval":41250}}"#.to_string();
        let payload: TestPayload = from_string(json).expect("Failed to parse");

        assert_eq!(payload.op, 10);
        assert_eq!(payload.d.unwrap().heartbeat_interval, 41250);
    }

    #[test]
    fn test_parse_null_data() {
        let json = r#"{"op":11,"d":null}"#.to_string();
        let payload: TestPayload = from_string(json).expect("Failed to parse");

        assert_eq!(payload.op, 11);
        assert!(payload.d.is_none());
    }

    #[test]
    fn test_serialize() {
        #[derive(serde::Serialize)]
        struct Heartbeat {
            op: u8,
            d: Option<u64>,
        }

        let hb = Heartbeat { op: 1, d: Some(42) };
        let json = to_string(&hb).expect("Failed to serialize");

        assert!(json.contains(r#""op":1"#));
        assert!(json.contains(r#""d":42"#));
    }
}
