#![feature(once_cell)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::sync::LazyLock;

use anyhow::Result;
use num::BigUint;

mod elliptic_curve;
mod euclidean;
mod point;
#[cfg(test)] mod tests;

pub(crate) static ZERO: LazyLock<BigUint> = LazyLock::new(|| BigUint::from(0u32));
pub(crate) static ONE: LazyLock<BigUint> = LazyLock::new(|| BigUint::from(1u32));
pub(crate) static TWO: LazyLock<BigUint> = LazyLock::new(|| BigUint::from(2u32));
pub(crate) static THREE: LazyLock<BigUint> = LazyLock::new(|| BigUint::from(3u32));
