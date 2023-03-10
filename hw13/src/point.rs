use std::{
  ops::{Add, Deref, Mul, Neg, Sub},
  sync::LazyLock,
};

use anyhow::{anyhow, Result};
use num::BigUint;

use crate::{elliptic_curve::EllipticCurve, euclidean::invert_eea, ONE, THREE, TWO, ZERO};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
  pub point: Option<NonInfinitePoint>,
  pub curve: EllipticCurve,
}

impl Deref for Point {
  type Target = NonInfinitePoint;

  fn deref(&self) -> &Self::Target { self.point.as_ref().unwrap() }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonInfinitePoint {
  pub x: BigUint,
  pub y: BigUint,
}

impl NonInfinitePoint {
  fn reduce_modulo(self, modulus: &BigUint) -> Self {
    Self { x: self.x % modulus, y: self.y % modulus }
  }
}

impl Point {
  pub fn new(curve: EllipticCurve, coords: Option<NonInfinitePoint>) -> Result<Self> {
    match coords {
      Some(point) =>
        if curve.contains_inner(&point) {
          let point = Point { point: Some(point.reduce_modulo(&curve.modulus)), curve };
          Ok(point)
        } else {
          Err(anyhow!("Point not on curve"))
        },
      None => Ok(Point { point: None, curve }),
    }
  }

  pub fn is_infinite(&self) -> bool { self.point.is_none() }

  pub fn to_inner(&self) -> Option<&NonInfinitePoint> { self.point.as_ref() }
}

impl<'a> Neg for &'a Point {
  type Output = Point;

  fn neg(self) -> Self::Output {
    let point = self.point.as_ref().map(|point| NonInfinitePoint {
      x: point.x.clone(),
      y: &self.curve.modulus - point.y.clone(),
    });
    Point { point, curve: self.curve.clone() }
  }
}

impl Add for Point {
  type Output = Result<Point>;

  fn add(self, rhs: Self) -> Self::Output {
    if self.curve != rhs.curve {
      panic!("Cannot add points on different curves")
    }

    // 0 + Q = Q
    if self.is_infinite() {
      return Ok(rhs);
      // P + 0 = P
    } else if rhs.is_infinite() {
      return Ok(self);
      // P - P = 0
    } else if self == -&rhs {
      return Ok(Point::new(self.curve, None).unwrap());
      // P + P =
    }

    let p = &self.curve.modulus;
    let ll: BigUint = if self == rhs {
      // handle zero edge-case
      if self.y == *ZERO {
        return Ok(Point::new(self.curve, None).unwrap());
      }
      let l = TWO.clone() * self.y.clone();
      let l_inv: BigUint = invert_eea(l, p.clone());
      (&THREE.clone() * &self.x.pow(2) + TWO.clone() * self.curve.a2 * &self.x + self.curve.a4)
        * l_inv
    } else {
      let l_inv = invert_eea(&rhs.x - &self.x, p.clone());
      (&rhs.y - &self.y) * l_inv
    };

    let x = ll.pow(2) - self.curve.a2 - &self.x - &rhs.x;
    let y = ll * (&self.x - &x) - &self.y;
    Point::new(self.curve.clone(), Some(NonInfinitePoint { x: x % p, y: y % p }))
  }
}

impl Sub for Point {
  type Output = Result<Point>;

  fn sub(self, rhs: Self) -> Self::Output { self + -&rhs }
}
