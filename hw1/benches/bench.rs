//! Benches. To use, import functions of interest, and `cargo bench`.
//!
//! `iai` is an experimental benchmark harness, using Cachegrind to ferform precise single-shot
//! measurements. https://bheisler.github.io/criterion.rs/book/iai/iai.html
#![allow(unused_imports)]
use std::{thread, time};

use criterion::{black_box as bb, criterion_group, criterion_main, Criterion};
use uncloak_hw::{rsa_decrypt, rsa_encrypt, setup, Setup};

// 260 microseconds on my machine
fn bench_en(c: &mut Criterion) {
  let setup = setup();
  c.bench_function("encrypt", |f| {
    f.iter(|| rsa_encrypt(setup.pub_key.clone(), b"some_text", rand::thread_rng()))
  });
}

// 2.2 millisecond on my machine
fn bench_de(c: &mut Criterion) {
  let setup = setup();
  let ciphertext: Vec<u8> = rsa_encrypt(setup.pub_key, b"some text", rand::thread_rng());
  c.bench_function("decrypt", |f| f.iter(|| rsa_decrypt(setup.priv_key.clone(), &ciphertext)));
}

criterion_group!(benches, bench_en, bench_de);
criterion_main!(benches);
