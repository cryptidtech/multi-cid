// SPDX-License-Identifier: Apache-2.0

/// Errors created by this library
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// A multicodec decoding error
    #[error(transparent)]
    Multicodec(#[from] multi_codec::Error),
    /// A multihash error
    #[error(transparent)]
    Multihash(#[from] multi_hash::Error),
    /// Cid error
    #[error(transparent)]
    Cid(#[from] CidError),
}

/// Cid Errors created by this library
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CidError {
    /// Missing target codec
    #[error("Missing target data encoding codec")]
    MissingTargetCodec,
    /// Missing hash data
    #[error("Missing hash data")]
    MissingHash,
    /// Trying to build a legacy Cid using the wrong function
    #[error("Building legacy Cid with the wrong function")]
    LegacyCid,
    /// Trying to build a modern Cid using the wrong function
    #[error("Building modern Cid with the wrong function")]
    ModernCid,
}

impl Error {
    /// Get the error kind as a string
    #[must_use]
    pub const fn kind(&self) -> &str {
        match self {
            Self::Multicodec(_) => "Multicodec",
            Self::Multihash(_) => "Multihash",
            Self::Cid(_) => "Cid",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_kind() {
        let err = Error::Cid(CidError::MissingHash);
        assert_eq!(err.kind(), "Cid");
    }

    #[test]
    fn test_error_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<Error>();
        assert_sync::<Error>();
    }
}
