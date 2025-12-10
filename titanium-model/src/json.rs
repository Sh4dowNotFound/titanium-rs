//! JSON serialization abstraction.
//!
//! Provides a unified interface for `simd-json` (default) and `serde_json`.

#[cfg(feature = "simd")]
pub use simd_json::json;
#[cfg(feature = "simd")]
pub use simd_json::BorrowedValue;
#[cfg(feature = "simd")]
pub use simd_json::Error;
#[cfg(feature = "simd")]
pub use simd_json::OwnedValue as Value;

#[cfg(all(not(feature = "simd"), feature = "serde"))]
pub use serde_json::json;
#[cfg(all(not(feature = "simd"), feature = "serde"))]
pub use serde_json::Error;
#[cfg(all(not(feature = "simd"), feature = "serde"))]
pub use serde_json::Value;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[cfg(feature = "simd")]
pub fn from_str<T: DeserializeOwned>(json: &str) -> Result<T, Error> {
    // SIMD-JSON requires mutable access. We clone to ensure safety and compatibility
    // with immutable generic interfaces, though this incurs a copy cost.
    // Use `from_mut_str` or `from_string` for zero-copy/in-place performance.
    let mut buffer = json.as_bytes().to_vec();
    simd_json::from_slice(&mut buffer)
}

#[cfg(all(not(feature = "simd"), feature = "serde"))]
pub fn from_str<T: DeserializeOwned>(json: &str) -> Result<T, Error> {
    serde_json::from_str(json)
}

#[cfg(feature = "simd")]
pub fn to_string<T: Serialize>(value: &T) -> Result<String, Error> {
    simd_json::to_string(value)
}

#[cfg(all(not(feature = "simd"), feature = "serde"))]
pub fn to_string<T: Serialize>(value: &T) -> Result<String, Error> {
    serde_json::to_string(value)
}

#[cfg(feature = "simd")]
pub fn to_value<T: Serialize>(value: T) -> Result<Value, Error> {
    let mut bytes = simd_json::to_vec(&value)?;
    simd_json::from_slice(&mut bytes)
}

#[cfg(feature = "simd")]
pub fn to_borrowed_value(json: &mut [u8]) -> Result<BorrowedValue<'_>, Error> {
    simd_json::to_borrowed_value(json)
}

#[cfg(feature = "simd")]
pub fn from_value<T: DeserializeOwned>(value: Value) -> Result<T, Error> {
    T::deserialize(value)
}

#[cfg(feature = "simd")]
/// Parse JSON from a mutable slice (Zero-Copy).
///
/// This modifies the input buffer in-place to avoid allocations.
pub fn from_slice_mut<'a, T: Deserialize<'a>>(json: &'a mut [u8]) -> Result<T, Error> {
    simd_json::from_slice(json)
}

#[cfg(feature = "simd")]
pub fn from_borrowed_value<'a, T: Deserialize<'a>>(value: BorrowedValue<'a>) -> Result<T, Error> {
    T::deserialize(value)
}

#[cfg(all(not(feature = "simd"), feature = "serde"))]
pub fn to_value<T: Serialize>(value: T) -> Result<Value, Error> {
    serde_json::to_value(value)
}

#[cfg(all(not(feature = "simd"), feature = "serde"))]
pub fn from_slice_mut<T: DeserializeOwned>(json: &mut [u8]) -> Result<T, Error> {
    serde_json::from_slice(json)
}
