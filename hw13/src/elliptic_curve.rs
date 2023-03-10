use std::{cell::LazyCell, ops::Deref, sync::LazyLock};

use anyhow::Result;
use num::BigUint;

use crate::{point::{NonInfinitePoint, Point}, TWO, THREE};


/// toy elliptic curve, ported from
/// https://github.com/cjeudy/EllipticCurves/blob/master/EC.py to Rust

#[derive(Debug, Clone)]
pub struct EllipticCurve {
  pub name:           String,
  pub order:          BigUint,
  pub modulus:        BigUint,
  pub a2:             usize,
  pub a4:             usize,
  pub a6:             usize,
  pub is_weierstrass: bool,
}

impl EllipticCurve {
  fn new(name: String, order: BigUint, modulus: BigUint, coefficients: &[usize]) -> Self {
    // EllipticCurve { name, order, modulus, coefficients: coefficients.to_vec() }
    EllipticCurve {
      name,
      order,
      modulus,
      a2: coefficients[0],
      a4: coefficients[1],
      a6: coefficients[2],
      is_weierstrass: coefficients[0] == 0,
    }
  }

  fn discriminant(&self) -> i128 {
    let b2 = 4 * self.a2;
    let (b4, b6, b8) = (2 * self.a4, 4 * self.a6, b2 * self.a6 - self.a4.pow(2));
    let delta_summand = (9 * b2 * b4 * b6) as i128;
    let delta_negand = (b8 * b2.pow(2) + 8 * b4.pow(3) + 27 * b6.pow(2)) as i128;
    delta_summand - delta_negand
  }

  fn is_smooth(&self) -> bool { self.discriminant() != 0 }

  // does curve contain point?
  pub fn contains(&self, point: &Point) -> bool {
    if point.is_infinite() {
      return true;
    }
    self.contains_inner(point)
  }

  pub fn contains_inner(&self, point: &NonInfinitePoint) -> bool {
    let p = &self.modulus;
    let lhs = point.y.modpow(&TWO, p);
    let rhs =
      point.x.modpow(&THREE, p) + self.a2 * point.x.modpow(&TWO, p) + self.a4 * &point.x + self.a6;
    lhs == rhs
  }
}

impl PartialEq for EllipticCurve {
  fn eq(&self, other: &Self) -> bool {
    self.a2 == other.a2
      && self.a4 == other.a4
      && self.a6 == other.a6
      && self.modulus == other.modulus
  }
}
