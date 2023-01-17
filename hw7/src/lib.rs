#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::{
  fmt::Debug,
  ops::{Div, Rem},
};

use anyhow::Result;

#[derive(Clone, Debug, Default, PartialEq)]
struct Pair {
  a: usize,
  b: usize,
}

impl Pair {
  fn new(a: usize, b: usize) -> Self {
    assert!(a != 0 && b != 0);
    let (a, b) = (std::cmp::max(a, b), std::cmp::min(a, b));

    Self { a, b }
  }

  /// Obtain the greatest common denominator of the Pair. The algorithm:
  /// a = b*q1 + r1
  /// b = r1*q2 + r2
  /// r1 = r2*q3 + r3
  /// ...
  /// r{n-2} = r{n-1}*qn + 0
  ///
  /// Output r{n-1}.
  ///
  /// Any divisor of a and b is also a divisor of r1, r2,..., r{n-2}, thus r{n-1} is the greatest
  /// common divisor.
  ///
  /// Both an imperative and declarative implementation are given, I find the imperative algorithm
  /// easier to read.
  pub fn euclidean_algorithm(&self) -> usize {
    let (mut a, mut b) = (self.a, self.b);
    let mut r = a % b;
    if r == 0 {
      return b;
    }

    std::iter::repeat_with(|| {
      a = b;
      b = r;
      r = a % b;
      (b, r)
    })
    // when r = 0, b is the last non-zero remainder.
    .find(|(_,r)| *r == 0)
    .unwrap()
    .0 // return b
  }

  #[allow(unused_assignments)]
  pub fn euclidean_algorithm_imperative(&self) -> usize {
    let mut a = self.a;
    let mut b = self.b;
    let mut r = a % b;
    while r != 0 {
      a = b;
      b = r;
      r = b % r;
    }
    b
  }

  /// The Extended Euclidean Algorithm obtains values for `x` and `y` such that
  /// `ax + by = gcd(a,b)`. This implmentation is useful reference, but essentially unreadable.
  /// See https://brilliant.org/wiki/extended-euclidean-algorithm/ for reference.
  pub fn extended_euclidean_iterative(&self) -> (usize, isize, isize) {
    let (mut a, mut b) = (self.a as isize, self.b as isize);
    let (mut x, mut y) = (0, 1);
    let (mut u, mut v) = (1, 0);
    while a != 0 {
      let q = b / a;
      let r = b % a;
      let m = x - u * q;
      let n = y - v * q;
      b = a;
      a = r;
      x = u;
      y = v;
      u = m;
      v = n;
    }
    // gcd, x, y
    (b as usize, x, y)
  }

  pub fn extended_euclidean_recursive(&self) -> (usize, isize, isize) {
    // step: a =
    let (a, b) = (self.a as isize, self.b as isize);
    let (prev_u, prev_v) = (1, 0);
    let (u, v) = (0, 1);

    let (q, r, u, v, prev_u, prev_v) = Self::eea_inner(a, b, u, v, prev_u, prev_v);
    dbg!(q, r, u, v, prev_u, prev_v);
    if r == u * a + -v * b {
      (r as usize, u, -v)
    } else if r == -u * a + v * b {
      (r as usize, -u, v)
    } else {
      panic!();
    }
  }

  fn eea_inner(
    q: isize,
    r: isize,
    u: isize,
    v: isize,
    prev_u: isize,
    prev_v: isize,
  ) -> (isize, isize, isize, isize, isize, isize) {
    let next_r = q % r;
    if next_r == 0 {
      (q, r, u, v, prev_u, prev_v)
    } else {
      let next_q = q / r;
      let next_u = next_q * u + prev_u;
      let next_v = next_q * v + prev_v;
      // dbg!(r, next_r, next_u, next_v, u, v);
      Self::eea_inner(r, next_r, next_u, next_v, u, v)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_pair() {
    let pair = Pair::new(153, 87);
    let other_pair = Pair::new(87, 153);
    assert_eq!(pair, other_pair);
    assert_eq!(3, pair.euclidean_algorithm());
    assert_eq!((3, 4, -7), pair.extended_euclidean_iterative());
    assert_eq!((3, 4, -7), pair.extended_euclidean_recursive());
  }
}
