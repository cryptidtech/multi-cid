// SPDX-License-Identifier: Apache-2.0
//! # multi-cid
//!
//! Content Identifier (CID) implementation compatible with IPFS and IPLD.
//!
//! ## Overview
//!
//! This crate provides CID (Content Identifier) support for self-describing content
//! addresses. CIDs are used extensively in IPFS and IPLD to create cryptographically
//! secure, content-addressed links.
//!
//! This crate contains only the `Cid` half of the former `bs-multicid` crate. The
//! `Vlad` half lives in the standalone `multi-vlad` crate. The split lets a
//! downstream crate depend on only the type it needs: `multi-cid` does not pull in
//! `multi-key` or `multi-sig`, which were only required by `Vlad`.
//!
//! ## CID Versions
//!
//! - **CID v0**: Legacy format, base58btc encoded, implicit dag-pb codec
//! - **CID v1**: Modern format, supports multibase, explicit codec and version
//!
//! ## Quick Start
//!
//! ### Creating a CID
//!
//! ```rust
//! use multi_cid::cid;
//! use multi_codec::Codec;
//! use multi_hash::Builder as MhBuilder;
//!
//! // Create a multihash
//! let hash = MhBuilder::new_from_bytes(Codec::Sha2256, b"hello world")
//!     .unwrap()
//!     .try_build()
//!     .unwrap();
//!
//! // Create a CID v1
//! let cid = cid::Builder::new(Codec::Sha2256)
//!     .with_target_codec(Codec::DagCbor)
//!     .with_hash(&hash)
//!     .try_build()
//!     .unwrap();
//! ```
//!
//! ### Encoding and Decoding
//!
//! ```rust
//! use multi_cid::cid;
//! use multi_codec::Codec;
//! use multi_hash::Builder as MhBuilder;
//!
//! let hash = MhBuilder::new_from_bytes(Codec::Sha2256, b"data")
//!     .unwrap()
//!     .try_build()
//!     .unwrap();
//!
//! let cid = cid::Builder::new(Codec::Sha2256)
//!     .with_target_codec(Codec::DagCbor)
//!     .with_hash(&hash)
//!     .try_build()
//!     .unwrap();
//!
//! // CID created successfully
//! let bytes: Vec<u8> = cid.into();
//! assert!(!bytes.is_empty());
//! ```
//!
//! ## Features
//!
//! - **`serde`** (default): Enables serde serialization
//! - **`dag_cbor`**: Enables CBOR tag 42 support for DAG-CBOR links via `multi-cbor`
//!
//! ## DAG-CBOR Support
//!
//! When the `dag_cbor` feature is enabled, CIDs can be serialized with CBOR tag 42
//! for use in IPLD DAG-CBOR documents.

#![warn(missing_docs)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
// Pedantic/nursery/cargo lints are enabled in `[lints.clippy]` in Cargo.toml.
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

/// Errors produced by this library
pub mod error;
pub use error::{CidError, Error};

/// Cid content identifier types
pub mod cid;
pub use cid::{Cid, EncodedCid, LegacyEncodedCid};

/// Type-safe wrappers for CID components
pub mod types;
pub use types::{CidVersion, ContentType};

/// Serde serialization for Cid
#[cfg(feature = "serde")]
pub mod serde;

/// Commonly used items
///
/// ```
/// use multi_cid::prelude::*;
/// use multi_hash::Builder as MhBuilder;
///
/// let hash = MhBuilder::new_from_bytes(Codec::Sha2256, b"test")
///     .unwrap()
///     .try_build()
///     .unwrap();
/// let cid = cid::Builder::new(Codec::Sha2256)
///     .with_target_codec(Codec::DagCbor)
///     .with_hash(&hash)
///     .try_build()
///     .unwrap();
/// ```
pub mod prelude {
    pub use super::*;
    /// re-exports
    pub use multi_base::Base;
    pub use multi_codec::Codec;
    pub use multi_util::BaseEncoded;
}
