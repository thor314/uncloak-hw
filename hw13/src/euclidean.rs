use num::BigUint;

use crate::{ONE, ZERO};

pub fn invert_eea(a: BigUint, b: BigUint) -> BigUint {
  let (x, modulus) = if a > b { (b, a) } else { (a, b) };

  assert!(modulus > x && x > *ZERO);
  let a_col = [modulus.clone(), ONE.clone(), ZERO.clone()];
  let b_col = [x.clone(), ZERO.clone(), ONE.clone()];
  let mut transcript = vec![a_col.to_vec(), b_col.to_vec()];

  dbg!(transcript.clone());

  while transcript.iter().last().unwrap()[0] != *ONE {
    let last = transcript.last().unwrap();
    let sec_last = transcript.get(transcript.len() - 2).unwrap();
    let next_column = calculate_column(last, sec_last);

    transcript.push(next_column);
  }

  let last_col = transcript.last().unwrap();
  let candidate = last_col.last().unwrap().clone();
  println!("candidate: {}", &candidate * &x);
  if &candidate * &x % &modulus != *ONE {
    modulus - candidate
  } else {
    candidate
  }
}

fn calculate_column(last: &[BigUint], sec_last: &[BigUint]) -> Vec<BigUint> {
  let n = &sec_last[0] / &last[0];
  let r = &sec_last[0] - &last[0] * &n;
  let a = &n * &last[1] + &sec_last[1];
  let b = &n * &last[2] + &sec_last[2];
  vec![r, a, b]
}

fn update_step(a: &mut BigUint, old_a: &mut BigUint, quotient: &BigUint) {
  // dbg!(&a, &old_a, &quotient);
  let temp = a.clone();
  *a = &*old_a + quotient * &temp;
  *old_a = temp;
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{THREE, TWO};
  #[test]
  fn test_euclidean() {
    let eleven = BigUint::from(11u32);
    let three = BigUint::from(3u32);
    let four = BigUint::from(4u32);
    assert_eq!(four, invert_eea(three, eleven));
    let twentythree = BigUint::from(23u32);
    let five = BigUint::from(5u32);
    let fourteen = BigUint::from(14u32);
    assert_eq!(fourteen, invert_eea(five, twentythree));
  }
}
