# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-08-18

### Changed

- Migrated from the `bs-*` workspace path dependencies to the published crates.io `multi-*` crates. `multi-codec` is now `1.2`, `multi-hash` is now `1.1.2`, and the `multi-base`/`multi-trait`/`multi-util`/`multi-cbor` deps now resolve from crates.io. The `bs-multibase`/`bs-multicodec`/`bs-multihash`/`bs-multitrait`/`bs-multiutil` `package` aliases and `path` overrides are removed.
- `EncodedCid` is now `BaseEncoded<Cid, CidEncoder>` instead of `BaseEncoded<Cid, DetectedEncoder>`. This is a breaking change to the public type identity of `EncodedCid`.
- `From<Cid> for Vec<u8>` and the `serde` serialization path no longer delegate to `EncodeIntoBuffer` on `Codec`, `Multihash`, or `Cid`. They build the byte representation via `From<Codec> for Vec<u8>` and `From<Multihash> for Vec<u8>` plus `extend_from_slice`, because the published `multi-codec` and `multi-hash` types impl `From<*> for Vec<u8>` and `TryDecodeFrom` but not `EncodeIntoBuffer`.

### Added

- `CidEncoder`, a `multi_util::BaseEncoder` for CIDs. It tries multibase first, then `Base58Btc` explicitly for unprefixed v0 CIDs, then falls back to `multi_util::DetectedEncoder`. The published `DetectedEncoder` tries `Base58Flickr` before `Base58Btc` (alphabet iteration order) and bails on the first strict decode success; both base58 alphabets accept the same characters, so a naked base58btc v0 CID decoded under `Base58Flickr` first and produced wrong bytes. `CidEncoder` fixes v0 CID decoding.

### Fixed

- Unprefixed (naked) base58btc v0 CID strings now decode to the correct bytes. Previously they decoded as `Base58Flickr`, producing wrong bytes.

### Removed

- Removed the `ensure_no_std` job from `.github/workflows/rust.yml`. The crate is not `no_std`-compatible: `multi-hash` (a required dependency) hard-codes `unsigned-varint` with the `std` feature and has no `std` cargo feature or `#![no_std]` attribute, so a `thumbv6m-none-eabi` build fails with `E0463` in `subtle`, `constant_time_eq`, `data-encoding`, and `unsigned-varint`. This mirrors the fix applied upstream in `multi-hash`, which also has no `ensure_no_std` job.

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

### Fixed

- Adapted to the published crates.io APIs of `multi-codec`, `multi-hash`, and `multi-trait`. The published `Codec` and `Multihash` types impl `From<*> for Vec<u8>` and `TryDecodeFrom` but not `EncodeIntoBuffer`. The `From<Cid> for Vec<u8>` impl now builds the byte representation via `extend_from_slice` on `Vec<u8>` obtained from `From<Codec>` and `From<Multihash>`, instead of delegating to `EncodeIntoBuffer` on `Codec` and `Multihash`.
- Added a `CidEncoder` base encoder for `EncodedCid`. The published `multi-util` `DetectedEncoder` tries `Base58Flickr` before `Base58Btc` (alphabet iteration order) and bails on the first strict decode success. Both base58 alphabets accept the same characters, so a naked base58btc v0 CID string decoded under `Base58Flickr` first and produced wrong bytes. `CidEncoder` tries multibase first, then `Base58Btc` explicitly, then falls back to `DetectedEncoder` so v0 CIDs decode correctly. `EncodedCid` is now `BaseEncoded<Cid, CidEncoder>` instead of `BaseEncoded<Cid, DetectedEncoder>`.

### Notes

- The `multi-base`, `multi-codec`, `multi-hash`, `multi-trait`, and `multi-util` dependencies use the published crates.io versions.

[0.2.0]: https://github.com/cryptidtech/multi-cid/releases/tag/v0.2.0

[0.1.0]: https://github.com/cryptidtech/multi-cid/releases/tag/v0.1.0
