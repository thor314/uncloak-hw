#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// https://bheisler.github.io/criterion.rs/book/index.html

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hw5::secret_data_branching::*;
use once_cell::sync::Lazy;
// pub static SECRET_VEC: Lazy<Vec<u64>> = Lazy::new(|| vec![SECRET; 100]);

pub fn left(c: &mut Criterion) {
  c.bench_function("left", |b| b.iter(|| branches_on_secret(black_box(SECRET))));
}
pub fn right(c: &mut Criterion) {
  c.bench_function("right", |b| b.iter(|| branches_on_secret(black_box(0))));
}
pub fn vleft(c: &mut Criterion) {
  c.bench_function("vleft", |b| b.iter(|| branches_on_secret_vec(black_box(SECRET_VEC.clone()))));
}
pub fn vright(c: &mut Criterion) {
  c.bench_function("vright", |b| b.iter(|| branches_on_secret_vec(black_box(vec![0]))));
}

criterion_group!(benches, left, right, vleft, vright);
criterion_main!(benches);
