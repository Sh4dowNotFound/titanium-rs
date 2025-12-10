//! Snowflake ID type for Discord
//!
//! Discord uses 64-bit unsigned integers for unique identifiers,
//! but serializes them as strings in JSON to avoid precision loss.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// A Discord Snowflake ID.
///
/// Snowflakes are unique 64-bit unsigned integers used by Discord.
/// They are serialized as strings in JSON to prevent precision loss
/// in languages with limited integer precision (JavaScript).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Snowflake(pub u64);

impl Snowflake {
    /// Create a new Snowflake from a u64 value.
    #[inline]
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the raw u64 value.
    #[inline]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Extract the timestamp from this Snowflake.
    ///
    /// Returns milliseconds since Discord Epoch (2015-01-01T00:00:00Z).
    #[inline]
    pub const fn timestamp(self) -> u64 {
        (self.0 >> 22) + 1420070400000
    }

    /// Extract the internal worker ID.
    #[inline]
    pub const fn worker_id(self) -> u8 {
        ((self.0 & 0x3E0000) >> 17) as u8
    }

    /// Extract the internal process ID.
    #[inline]
    pub const fn process_id(self) -> u8 {
        ((self.0 & 0x1F000) >> 12) as u8
    }

    /// Extract the increment (sequence number within the same millisecond).
    #[inline]
    pub const fn increment(self) -> u16 {
        (self.0 & 0xFFF) as u16
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for Snowflake {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Snowflake> for u64 {
    #[inline]
    fn from(snowflake: Snowflake) -> Self {
        snowflake.0
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Always serialize as string to match Discord's format
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Discord sends snowflakes as strings, but we also handle integers
        struct SnowflakeVisitor;

        impl<'de> serde::de::Visitor<'de> for SnowflakeVisitor {
            type Value = Snowflake;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or integer snowflake ID")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Snowflake(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Snowflake(value as u64))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                value
                    .parse::<u64>()
                    .map(Snowflake)
                    .map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(SnowflakeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snowflake_parsing() {
        let json_str = r#""175928847299117063""#;
        let snowflake: Snowflake = crate::json::from_str(json_str).unwrap();
        assert_eq!(snowflake.get(), 175928847299117063);
    }

    #[test]
    fn test_snowflake_serialization() {
        let snowflake = Snowflake::new(175928847299117063);
        let json = crate::json::to_string(&snowflake).unwrap();
        assert_eq!(json, r#""175928847299117063""#);
    }

    #[test]
    fn test_snowflake_timestamp() {
        // Known snowflake with known timestamp
        let snowflake = Snowflake::new(175928847299117063);
        assert!(snowflake.timestamp() > 1420070400000);
    }
}
