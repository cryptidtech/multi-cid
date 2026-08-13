# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-08-12

### Added

- Initial standalone release of `multi-cid` on crates.io.
- `Cid`, `EncodedCid`, `LegacyEncodedCid`, `CidVersion`, `ContentType`, the `Cid` builder, and `CidError`. Re-exported from the crate root.
- `serde` cargo feature (default). Human-readable formats give a struct (`version`, `encoding`, `hash`); binary formats give the raw bytes.
- `dag_cbor` cargo feature (default). Enables CBOR tag 42 support for DAG-CBOR links via `multi-cbor`. When the feature is on, `Cid` serializes as a CBOR tag 42 byte string with the leading `Identity` codec per the [DAG-CBOR spec](https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-cbor.md#links). When the feature is off, `Cid` serializes as plain bytes.
- `[lints.clippy]` config in `Cargo.toml` with `pedantic`, `nursery`, and `cargo` groups.

### Changed

- Extracted from the `bs-multicid` workspace crate (`bettersign/crates/multicid/`). The crate is renamed from `bs-multicid` to `multi-cid`. All `use bs_multicid::...` references now use `use multi_cid::...`.
- The `Vlad` type and its dependencies (`multi-key`, `multi-sig`) are removed. `Vlad` now lives in the standalone `multi-vlad` crate. The `Error` enum no longer has `Multikey`, `Multisig`, or `Vlad` variants. The `VladError` enum is removed.
- The `Error::kind()` method no longer returns `"Multikey"`, `"Multisig"`, or `"Vlad"`.
- The `serde` module no longer has `Serialize`/`Deserialize` impls for `Vlad`. The `serde` tests for `Vlad` are removed.
- The `prelude` module re-exports `multi_base::Base`, `multi_codec::Codec`, and `multi_util::BaseEncoded` (renamed from `bs_multibase`, `bs_multicodec`, `bs_multiutil`).
- The `dag_cbor` feature now uses `multi-cbor` (the standalone fork of `serde_cbor`) instead of the workspace `serde_cbor`. The serde impls reference `multi_cbor::tags::Tagged` and `multi_cbor::value::Value`.
- The bench file no longer benchmarks `Vlad` encoding. Only `Cid` encoding and decoding are benchmarked.
- The `repository` field points at `https://github.com/cryptidtech/multi-cid.git`.
- The MSRV is declared as 1.85.

### Notes

- The `multi-base`, `multi-codec`, `multi-hash`, `multi-trait`, and `multi-util` dependencies currently point at the `bs-*` workspace path deps in `bettersign/crates/` via `package` rename. The published crates.io versions of `multi-codec` and `multi-hash` lack the `EncodeIntoBuffer` and `TryDecodeFrom` trait impls that `Cid` needs. When those impls are published, the path deps will switch to the crates.io versions.

[0.1.0]: https://github.com/cryptidtech/multi-cid/releases/tag/v0.1.0