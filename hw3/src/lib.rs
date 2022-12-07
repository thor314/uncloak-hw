#![allow(dead_code)]
mod ch5_ring {
  use std::collections::HashMap;

  use rand::Rng;
  use ring::digest;

  const BYTES: usize = 2;

  /// Truncate the output of SHA3-512(message) to n bits
  fn sha_512_n(m: &[u8], n_bytes: usize) -> Vec<u8> {
    assert!((1..=8).contains(&n_bytes));
    let mut ctx = digest::Context::new(&digest::SHA512);
    ctx.update(m);
    ctx.finish().as_ref()[..n_bytes].to_vec()
  }

  /// write a birthday attack on SHA3-512-n; see benches for timings
  fn exercise5_3(n_bytes: usize) -> (usize, usize) {
    assert!((1..=8).contains(&n_bytes));

    let mut map = HashMap::new();
    let (i1, i2, _hash) = (1usize..100_000)
      .find_map(|i| {
        let hash = sha_512_n(&i.to_be_bytes(), n_bytes);
        map.insert(hash.clone(), i).map(|i2| (i, i2, hash))
      })
      .unwrap();

    // println!("i1: {i1:?}");
    // println!("i2: {i2:?}");
    // println!("hash: {_hash:?}");
    (i1, i2)
  }

  /// Find a message that hashes to "3D4B" using the previous exercise and input size 16 bits.
  fn exercise5_4(s: &[u8], start: usize) -> usize {
    assert!(s.len() == BYTES);
    let count = (start..(start + 1000000))
      .find(|i| {
        let hash = sha_512_n(&i.to_be_bytes(), BYTES);
        hash == s
      })
      .unwrap();

    // let hash = sha_512_n(&count.to_be_bytes(), BYTES);
    // println!("count: {count:?} hashes to: {hash:?}");
    count - start
  }

  fn average_iters_5_4(n_reps: usize, s: &[u8]) -> usize {
    let mut rng = rand::thread_rng();
    // divide by 100 to avoid overflow errors (lol)
    let sum = (1..=n_reps).fold(0, |sum, _| sum + exercise5_4(s, rng.gen::<usize>() / 100));
    // dbg!(&s);
    sum / n_reps
  }
  #[cfg(test)]
  mod tests {

    use super::*;

    #[test]
    fn ch5_test() {
      let (i1, i2) = exercise5_3(BYTES);
      assert!(sha_512_n(&i1.to_be_bytes(), BYTES) == sha_512_n(&i2.to_be_bytes(), BYTES));

      let s = [0x3D, 0x4B];
      let n = exercise5_4(&s,0);
      assert_eq!(n, 38804);

      // expect about 2^16=65536 iters.
      let a = average_iters_5_4(5, &s);
      assert!(a < 100_000); // very unlikely
    }
  }
}
