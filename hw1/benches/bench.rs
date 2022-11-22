//! Benches. To use, import functions of interest, and `cargo bench`.
//!
//! https://bheisler.github.io/criterion.rs/book/iai/iai.html
#![allow(unused_imports)]
use std::{thread, time};

use criterion::{black_box as bb, criterion_group, criterion_main, Criterion};
use uncloak_hw::RSA;

// 260 microseconds on my machine
fn bench_en(c: &mut Criterion) {
  let rsa = RSA::new();

  c.bench_function("encrypt", |f| f.iter(|| rsa.encrypt(b"some_text", rand::thread_rng())));
}

// 2.2 millisecond on my machine
fn bench_de(c: &mut Criterion) {
  let rsa = RSA::new();
  let ciphertext: Vec<u8> = rsa.encrypt(b"some text", rand::thread_rng());
  c.bench_function("decrypt", |f| f.iter(|| rsa.decrypt(&ciphertext)));
}

criterion_group!(benches, bench_en, bench_de);
criterion_main!(benches);
