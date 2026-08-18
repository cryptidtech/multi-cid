[![](https://img.shields.io/badge/made%20by-Cryptid%20Technologies-gold.svg?style=flat-square)](https://cryptid.tech/)
[![](https://img.shields.io/badge/project-provenance-purple.svg?style=flat-square)](https://github.com/cryptidtech/provenance-specifications/)
[![](https://img.shields.io/badge/project-multiformats-blue.svg?style=flat-square)](https://github.com/multiformats/multiformats/)

[![Build Status](https://github.com/cryptidtech/multi-cid/actions/workflows/rust.yml/badge.svg)](https://github.com/cryptidtech/multi-cid/actions)
[![License](https://img.shields.io/crates/l/multi-cid?style=flat-square)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/multi-cid?style=flat-square)](https://crates.io/crates/multi-cid)
[![Documentation](https://docs.rs/multi-cid/badge.svg?style=flat-square)](https://docs.rs/multi-cid)

# multi-cid

Rust implementation of the [multiformats](https://github.com/multiformats/multiformats/) [Content Identifier (CID)](https://docs.ipfs.tech/concepts/content-addressing/) specification for IPFS and IPLD.

A CID is a self-describing content address. It pairs a version, a target codec, and a multihash of the content. This crate supports CIDv0 (legacy, base58btc, implicit dag-pb) and CIDv1 (modern, multibase, explicit codec).

This crate contains only the `Cid` half of the former `bs-multicid` workspace crate. The `Vlad` half lives in the standalone `multi-vlad` crate. The split lets a downstream crate depend on only the type it needs: `multi-cid` depends on `multi-hash` but not on `multi-key` or `multi-sig`, which were only required by `Vlad`.

## Table of Contents

- [Features](#features)
- [Install](#install)
- [Usage](#usage)
- [Feature Flags](#feature-flags)
- [The Split from `bs-multicid`](#the-split-from-bs-multicid)
- [Testing](#testing)
- [Maintainers](#maintainers)
- [Contribute](#contribute)
- [License](#license)

## Features

- CIDv0 and CIDv1 support.
- Builder pattern for CIDs from a codec, target codec, and multihash.
- Multibase encoding via `multi-base` and `multi-util`. `EncodedCid` detects the encoding on decode, so base58btc v0 CIDs and multibase v1 CIDs both round-trip.
- `LegacyEncodedCid` for bare base58btc-encoded v0 CIDs.
- Serde integration under the `serde` feature. Human-readable formats give a struct (`version`, `encoding`, `hash`); binary formats give the raw bytes.
- DAG-CBOR tag 42 support under the `dag_cbor` feature, via `multi-cbor`.
- Type-safe wrappers: `CidVersion` and `ContentType`.

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
multi-cid = "0.1"
```

For DAG-CBOR tag 42 support:

```toml
[dependencies]
multi-cid = { version = "0.1", features = ["dag_cbor"] }
```

MSRV: Rust 1.85 (Edition 2021).

## Usage

```rust
use multi_cid::cid;
use multi_codec::Codec;
use multi_hash::Builder as MhBuilder;

// Create a multihash
let hash = MhBuilder::new_from_bytes(Codec::Sha2256, b"hello world")
    .unwrap()
    .try_build()
    .unwrap();

// Create a CID v1
let cid = cid::Builder::new(Codec::Sha2256)
    .with_target_codec(Codec::DagCbor)
    .with_hash(&hash)
    .try_build()
    .unwrap();

// Encode to bytes and round-trip
let bytes: Vec<u8> = cid.clone().into();
let decoded = cid::Cid::try_from(&bytes[..]).unwrap();
assert_eq!(cid, decoded);
```

## Feature Flags

| Feature | Default | Effect |
|---|---|---|
| `serde` | yes | Enables serde serialization for `Cid`. |
| `dag_cbor` | yes | Enables CBOR tag 42 support for DAG-CBOR links via `multi-cbor`. Implies `serde`. |

## The Split from `bs-multicid`

The former `bs-multicid` workspace crate contained two types: `Cid` and `Vlad`. These types do not import from each other. They have disjoint dependency sets:

- `Cid` needs `multi-hash` for the `Multihash` field.
- `Vlad` needs `multi-key` and `multi-sig` for the inner `Multisig`.

Splitting the crate lets a downstream crate depend on only the type it needs. For example, `wacc` needs `Cid` but not `Vlad`, so `wacc` does not pull in `multi-key` or `multi-sig`.

The `multi-cid` crate is the `Cid` half. The `multi-vlad` crate is the `Vlad` half.

## Testing

```bash
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo doc --all-features --no-deps
```

## Maintainers

- Dave Grantham <dwg@linuxprogrammer.org>

## Contribute

Pull requests go to the [`cryptidtech/multi-cid`](https://github.com/cryptidtech/multi-cid)
repository. Sign commits with GPG. Use Conventional Commits messages.

## License

Licensed under `Apache-2.0`.

See [`LICENSE`](LICENSE) for the full text.