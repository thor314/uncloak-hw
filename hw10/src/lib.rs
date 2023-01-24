#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use anyhow::Result;

mod error;
#[cfg(test)] mod tests;
mod utils;

use num::{bigint::BigUint, one, Zero};

// kfn fast_power(n: &BigUint, e: usize, modulo: &BigUint) -> BigUint {
//   // obtain the (little-endian) binary representation of e
//   let e_bits = format!("{e:b}").chars().map(|c| c == '1').rev().collect::<Vec<bool>>();

//   // powers of two (little first, 1, 2, 4,...)
//   let two = BigUint::from(2u8);
//   let mut state: BigUint = one();
//   let powers_of_two: Vec<BigUint> = {
//     let uno = std::iter::once(one());
//     let rest = std::iter::repeat_with(|| {
//       state = &state * &two % modulo;
//       state.clone()
//     });
//     uno.chain(rest)
//   }
//   .take(e_bits.len())
//   .collect();
//   //   dbg!(&e_bits, &powers_of_two);

//   // filter and multiply the powers we want:
//   let product: BigUint = (0..e_bits.len())
//   // filter the 0-powers
//     .filter(|i| e_bits[*i])
//     .map(|i| powers_of_two[i].clone())
//     .inspect(| x: &BigUint| { dbg!(x); })
//     .map(|i| n ** &i % modulo)
//     .product();
//     product % modulo
// }

/// For a more production-ready implementation, see rust-num:
/// https://github.com/rust-num/num-bigint/blob/master/src/biguint/power.rs#L149
fn fast_power(n: &BigUint, e: usize, modulo: &BigUint) -> BigUint {
  // handle dumb inputs
  assert!(!n.is_zero());
  assert!(!modulo.is_zero());
  if e.is_zero() {
    return BigUint::from(1u8);
  }
  let n = n % modulo;

  // obtain the (little-endian) binary representation of e
  let e_bits: Vec<bool> = format!("{e:b}").chars().map(|c| c == '1').rev().collect();
  // obtain the powers of two to multiply
  let mut state = BigUint::from(1u8);
  let mut exp_state: BigUint = n;
  for bit in e_bits {
    if bit {
      state = state * &exp_state % modulo;
      dbg!(&state, &exp_state);
    }
    exp_state = &exp_state * &exp_state % modulo;
  }

  state
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_fast_pow() {
    let uno = fast_power(&BigUint::from(3u8), 3, &BigUint::from(26u16));
    assert_eq!(uno, BigUint::from(1u16));
    let uno = fast_power(&BigUint::from(2u8), 3, &BigUint::from(7u16));
    assert_eq!(uno, BigUint::from(1u16));
    let uno = fast_power(&BigUint::from(5u8), 4, &BigUint::from(624u16));
    assert_eq!(uno, BigUint::from(1u16));
    let uno = fast_power(&BigUint::from(7u8), 3, &BigUint::from(342u16));
    assert_eq!(uno, BigUint::from(1u16));
    let uno = fast_power(&BigUint::from(2u8), 16, &BigUint::from(65535u16));
    assert_eq!(uno, BigUint::from(1u16));
  }
}
