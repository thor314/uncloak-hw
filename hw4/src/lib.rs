use std::array;

use ring::aead::{self, BoundKey, Nonce, NonceSequence};

struct Counter {
  count: u64,
}

impl Counter {
  fn new() -> Counter { Counter { count: 0 } }
}
impl NonceSequence for Counter {
  fn advance(&mut self) -> std::result::Result<ring::aead::Nonce, ring::error::Unspecified> {
    self.count += 1;

    let mut arr = [0; 12];
    for (i, e) in self.count.to_be_bytes().into_iter().enumerate() {
      arr[i] = e
    }

    Ok(Nonce::assume_unique_for_key(arr))
  }
}

// docs: https://docs.rs/ring/latest/ring/aead/index.html
pub fn ring_encrypt_aead_gcm(m: &mut [u8; 12]) -> Vec<u8> {
  // We want to construct a Sealing Key, to encrypt and sign.
  let aead_unbound_key = aead::UnboundKey::new(&aead::AES_128_GCM, m).unwrap();
  let counter = Counter::new();
  let mut aead_sealing_key = aead::SealingKey::new(aead_unbound_key, counter);
  let aad = aead::Aad::from(&[]);

  let mut m = m.to_vec();
  aead_sealing_key.seal_in_place_append_tag(aad, &mut m).unwrap();
  m
}

pub fn add(left: usize, right: usize) -> usize { left + right }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ring_aead() {
    let mut m = [0; 12];
    let out = ring_encrypt_aead_gcm(&mut m);
  }
}
