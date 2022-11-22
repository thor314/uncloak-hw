//! A batteries-included library template.
// TODO: remove these when ready
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// use anyhow::Result;
use rand::{rngs::ThreadRng, CryptoRng, Rng, RngCore};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

const BITS: usize = 2048;

pub struct Setup {
  pub priv_key: RsaPrivateKey,
  pub pub_key:  RsaPublicKey,
}

pub fn setup() -> Setup {
  let priv_key = RsaPrivateKey::new(&mut rand::thread_rng(), BITS).expect("failed to generate a key");
  let pub_key = RsaPublicKey::from(&priv_key);
  Setup { priv_key, pub_key }
}

pub fn rsa_encrypt<R: CryptoRng + RngCore>(
  pub_key: RsaPublicKey,
  data: &[u8],
  mut rng: R,
) -> Vec<u8> {
  pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), data).expect("failed to encrypt")
  // assert_ne!(&data[..], &enc_data[..]);
}

pub fn rsa_decrypt(priv_key: RsaPrivateKey, enc_data: &[u8]) -> Vec<u8> {
  priv_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), enc_data).expect("failed to decrypt")
}
