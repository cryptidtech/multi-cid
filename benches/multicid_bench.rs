// SPDX-License-Identifier: Apache-2.0
//! Performance benchmarks for multi-cid

use criterion::{criterion_group, criterion_main, Criterion};
use multi_cid::{cid, Cid};
use multi_codec::Codec;
use multi_hash::Builder as MhBuilder;
use multi_trait::EncodeInto;
use std::hint::black_box;

/// Benchmark CID encoding
fn bench_cid_encoding(c: &mut Criterion) {
    let hash = MhBuilder::new_from_bytes(Codec::Sha2256, b"benchmark data")
        .unwrap()
        .try_build()
        .unwrap();

    let cid = cid::Builder::new(Codec::Cidv1)
        .with_target_codec(Codec::DagCbor)
        .with_hash(&hash)
        .try_build()
        .unwrap();

    c.bench_function("cid_to_bytes", |b| {
        b.iter(|| {
            let _bytes: Vec<u8> = black_box(&cid).encode_into();
        });
    });
}

/// Benchmark CID decoding
fn bench_cid_decoding(c: &mut Criterion) {
    let hash = MhBuilder::new_from_bytes(Codec::Sha2256, b"benchmark data")
        .unwrap()
        .try_build()
        .unwrap();

    let cid = cid::Builder::new(Codec::Cidv1)
        .with_target_codec(Codec::DagCbor)
        .with_hash(&hash)
        .try_build()
        .unwrap();
    let bytes: Vec<u8> = cid.into();

    c.bench_function("cid_from_bytes", |b| {
        b.iter(|| Cid::try_from(black_box(bytes.as_ref())));
    });
}

criterion_group!(benches, bench_cid_encoding, bench_cid_decoding);
criterion_main!(benches);
