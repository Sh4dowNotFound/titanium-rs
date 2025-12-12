//! Smart string implementation for Titan-rs.
//!
//! Provides a Copy-on-Write string type that supports:
//! - Borrowed slices (`&'a str`) for zero-copy parsing.
//! - Owned strings (`String`) for modification.
//! - Shared strings (`Arc<str>`) for efficient caching.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Eq)]
pub enum TitanString<'a> {
    Borrowed(&'a str),
    Owned(String),
    Shared(Arc<str>),
}

impl TitanString<'_> {
    #[must_use]
    pub fn into_owned(self) -> String {
        match self {
            Self::Borrowed(s) => s.to_owned(),
            Self::Owned(s) => s,
            Self::Shared(s) => s.to_string(),
        }
    }

    #[must_use]
    pub fn into_shared(self) -> Arc<str> {
        match self {
            Self::Borrowed(s) => Arc::from(s),
            Self::Owned(s) => Arc::from(s),
            Self::Shared(s) => s,
        }
    }

    /// Get a reference to the underlying string slice.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self
    }
}

impl Deref for TitanString<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(s) => s,
            Self::Owned(s) => s,
            Self::Shared(s) => s,
        }
    }
}

impl fmt::Debug for TitanString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl fmt::Display for TitanString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl PartialEq for TitanString<'_> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl PartialEq<str> for TitanString<'_> {
    fn eq(&self, other: &str) -> bool {
        &**self == other
    }
}

impl PartialEq<&str> for TitanString<'_> {
    fn eq(&self, other: &&str) -> bool {
        &**self == *other
    }
}

impl PartialEq<String> for TitanString<'_> {
    fn eq(&self, other: &String) -> bool {
        &**self == other.as_str()
    }
}

impl<'a> PartialEq<TitanString<'a>> for String {
    fn eq(&self, other: &TitanString<'a>) -> bool {
        self.as_str() == &**other
    }
}

impl std::hash::Hash for TitanString<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl Serialize for TitanString<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self)
    }
}

impl<'de> Deserialize<'de> for TitanString<'_> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(TitanString::Owned(s))
    }
}

impl<'a> From<&'a str> for TitanString<'a> {
    fn from(s: &'a str) -> Self {
        Self::Borrowed(s)
    }
}

impl From<String> for TitanString<'_> {
    fn from(s: String) -> Self {
        Self::Owned(s)
    }
}

impl From<Arc<str>> for TitanString<'_> {
    fn from(s: Arc<str>) -> Self {
        Self::Shared(s)
    }
}

impl<'a> From<Cow<'a, str>> for TitanString<'a> {
    fn from(cow: Cow<'a, str>) -> Self {
        match cow {
            Cow::Borrowed(s) => Self::Borrowed(s),
            Cow::Owned(s) => Self::Owned(s),
        }
    }
}

impl<'a> From<&'a String> for TitanString<'a> {
    #[inline]
    fn from(s: &'a String) -> Self {
        Self::Borrowed(s.as_str())
    }
}

impl Default for TitanString<'_> {
    fn default() -> Self {
        Self::Borrowed("")
    }
}
