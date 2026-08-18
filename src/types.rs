// SPDX-License-Identifier: Apache-2.0
//! Type-safe wrappers for CID components

use core::fmt;
use multi_codec::Codec;

/// CID version identifier
///
/// Type-safe wrapper for CID version (0 or 1).
///
/// # Examples
///
/// ```
/// use multi_cid::types::CidVersion;
///
/// let v0 = CidVersion::V0;
/// let v1 = CidVersion::V1;
/// assert_eq!(v0.as_u8(), 0);
/// assert_eq!(v1.as_u8(), 1);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CidVersion {
    /// CID version 0 (legacy, base58btc only)
    V0,
    /// CID version 1 (modern, multibase)
    V1,
}

impl CidVersion {
    /// Get version as u8
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::V0 => 0,
            Self::V1 => 1,
        }
    }

    /// Create from u8
    #[must_use]
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::V0),
            1 => Some(Self::V1),
            _ => None,
        }
    }

    /// Check if legacy (v0)
    #[must_use]
    pub const fn is_legacy(self) -> bool {
        matches!(self, Self::V0)
    }

    /// Check if modern (v1)
    #[must_use]
    pub const fn is_modern(self) -> bool {
        matches!(self, Self::V1)
    }
}

impl fmt::Display for CidVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.as_u8())
    }
}

/// Content type identifier
///
/// Type-safe wrapper for content codec.
///
/// # Examples
///
/// ```
/// use multi_cid::types::ContentType;
/// use multi_codec::Codec;
///
/// let content = ContentType::new(Codec::DagCbor);
/// assert_eq!(content.codec(), Codec::DagCbor);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContentType(Codec);

impl ContentType {
    /// Create new `ContentType`
    #[must_use]
    pub const fn new(codec: Codec) -> Self {
        Self(codec)
    }

    /// Get codec
    #[must_use]
    pub const fn codec(self) -> Codec {
        self.0
    }

    /// Get name
    #[must_use]
    pub fn name(self) -> &'static str {
        self.0.into()
    }

    /// Get code
    #[must_use]
    pub fn code(self) -> u64 {
        self.0.code()
    }
}

impl From<Codec> for ContentType {
    fn from(codec: Codec) -> Self {
        Self(codec)
    }
}

impl From<ContentType> for Codec {
    fn from(ct: ContentType) -> Self {
        ct.0
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cid_version() {
        assert_eq!(CidVersion::V0.as_u8(), 0);
        assert_eq!(CidVersion::V1.as_u8(), 1);

        assert_eq!(CidVersion::from_u8(0), Some(CidVersion::V0));
        assert_eq!(CidVersion::from_u8(1), Some(CidVersion::V1));
        assert_eq!(CidVersion::from_u8(2), None);
    }

    #[test]
    fn test_cid_version_predicates() {
        assert!(CidVersion::V0.is_legacy());
        assert!(!CidVersion::V0.is_modern());
        assert!(CidVersion::V1.is_modern());
        assert!(!CidVersion::V1.is_legacy());
    }

    #[test]
    fn test_cid_version_display() {
        assert_eq!(CidVersion::V0.to_string(), "v0");
        assert_eq!(CidVersion::V1.to_string(), "v1");
    }

    #[test]
    fn test_content_type() {
        let ct = ContentType::new(Codec::DagCbor);
        assert_eq!(ct.codec(), Codec::DagCbor);
        assert_eq!(ct.name(), "dag-cbor");
    }

    #[test]
    fn test_content_type_conversions() {
        let codec = Codec::DagJson;
        let ct = ContentType::from(codec);
        let back: Codec = ct.into();
        assert_eq!(back, codec);
    }

    #[test]
    fn test_newtypes_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<CidVersion>();
        assert_sync::<CidVersion>();
        assert_send::<ContentType>();
        assert_sync::<ContentType>();
    }
}
