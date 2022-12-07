#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod ch3 {
  // use anyhow::Result;
  use openssl::{
    error::ErrorStack,
    symm::{decrypt, encrypt, Cipher},
  };
  // with OpenSSL (https://docs.rs/openssl/0.10.43/openssl/encrypt/index.html)
  // alternatively, see AES: https://docs.rs/aes/0.8.2/aes/

  /// decrypt a ciphertext with aes256
  fn q8_openssl_decrypt(key: &[u8], c: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    decrypt(Cipher::aes_256_cbc(), key, None, c)
  }

  /// encrypt a plaintext with aes256
  fn q9_openssl_encrypt(key: &[u8], p: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    encrypt(Cipher::aes_256_cbc(), key, None, p)
  }

  /// demonstrate DES complementation
  fn q10_complementation(key: &[u8], p: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let kk = key.iter().map(|v| !v).collect();
    let pp = p.iter().map(|v| !v).collect();
    (kk, pp)
  }

  #[cfg(test)]
  mod tests3 {
    use aes::{cipher::BlockEncrypt, Block};
    use aes_gcm::KeyInit;
    use des::Des;
    use generic_array::GenericArray;

    use super::*;
    #[test]
    fn testy_test3() {
      let key = b"\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01";
      let c = b"\x53\x9B\x33\x3B\x39\x70\x6D\x14\x90\x28\xCF\xE1\xD9\xD4\xA4\x07";
      let p = q8_openssl_decrypt(key, c).unwrap();
      println!("3.8 plaintext: {p:?}\nciphertext: {c:?}");
      assert_eq!(c.to_vec(), q9_openssl_encrypt(key, &p).unwrap());

      let p9 = b"\x29\x6C\x93\xFD\xF4\x99\xAA\xEB\x41\x94\xBA\xBC\x2E\x63\x56\x1D";
      let c9 = q9_openssl_encrypt(key, p9).unwrap();
      println!("3.9 plaintext: {p9:?}\nciphertext: {c9:?}");
      assert_eq!(p9.to_vec(), q8_openssl_decrypt(key, &c9).unwrap());

      // 3.10
      // denote the complement of x to be x'.
      // does e(k,p) = e(k',p')'?
      use generic_array::typenum::consts::U8;
      let key = [1u8, 2, 3, 4, 1, 3, 8, 3];
      let key: &GenericArray<u8, U8> = GenericArray::from_slice(&key);
      // let p = "an8bitky".as_bytes();
      let p = [1u8, 4, 3, 8, 1, 3, 3, 4];
      let p = GenericArray::from_slice(&p);
      let mut c = [0u8; 8];
      let c = GenericArray::from_mut_slice(&mut c);
      let des = Des::new_from_slice(key).unwrap();
      des.encrypt_block_b2b(p, c);

      let (kk, pp) = q10_complementation(key, p);
      let pp = GenericArray::from_slice(&pp);
      let des = Des::new_from_slice(&kk).unwrap();
      let mut cc = [0u8; 8];
      let cc = GenericArray::from_mut_slice(&mut cc);

      des.encrypt_block_b2b(pp, cc);
      let cc_cmp: Vec<u8> = cc.into_iter().map(|v| !*v).collect();
      assert_eq!(c.to_vec(), cc_cmp.to_vec());

      // OpenSSL doesn't expose this function, or at least not with any clear error, so use another
      // crate msg needs a padding but I'm feeling lazy, shrugsies 🇺🇲
      // thread 'ch3::tests3::testy_test3' panicked at 'called `Result::unwrap()` on an `Err` value:
      // ErrorStack([Error { code: 50856204, library: "digital envelope routines", function:
      // "inner_evp_generic_fetch", reason: "unsupported", file: "../crypto/evp/evp_fetch.c", line:
      // 349, data: "Global default library context, Algorithm (DES-CBC : 8), Properties ()" }])',
      // hw2/src/lib.rs:58:64
      // let des_c = encrypt(Cipher::des_cbc(), key, Some(iv), p).unwrap();
      // let (kk, pp) = q10_complementation(key, &*p);
      // let des_cc = encrypt(Cipher::des_cbc(), &kk, Some(iv), &pp).unwrap();
      // let des_c2: Vec<u8> = des_cc.into_iter().map(|v| !v).collect();
      // println!("3.10 complementation comparison:\nc1: {des_c:?}\nc2: {des_c2:?}");
      // assert_eq!(des_c, des_c2);
    }
  }
}

mod ch4 {
  /// Array of length 256 with PKCS#7 padding
  #[derive(Clone, Debug)]
  struct PaddedMessage {
    pub msg: Vec<u8>,
  }

  /// implement pkcs#7
  fn pkcs7(msg: &[u8]) -> PaddedMessage {
    assert!(msg.len() < 256);
    let msg_len = msg.len() as u8;
    match msg_len {
      255 => panic!("message length must be < 255"),
      1..=254 => {
        let pad = 255 - msg_len;
        let pad_arr = vec![pad; pad as usize];
        let mut padded_msg = Vec::with_capacity(255);
        padded_msg.extend_from_slice(msg);
        padded_msg.extend(std::iter::repeat(pad).take(pad as usize));
        PaddedMessage { msg: padded_msg.to_vec() }
      },
      0 => panic!("message length must be > 0"),
    }
  }

  /// check whether padding length is correct
  fn pkcs7_validate(pm: &PaddedMessage) -> anyhow::Result<()> {
    let a: Vec<&u8> = pm.msg.iter().rev().take(2).collect();
    if pm.msg.iter().rev().take(*a[0] as usize).all(|x| x == a[0]) {
      Ok(())
    } else {
      Err(anyhow::anyhow!("yikes! padding error"))
    }
  }
  #[cfg(test)]
  mod tests4 {
    use super::*;
    macro_rules! try_pad {
      ($n:expr, $len:expr) => {
        let arr = vec![$n; $len];
        let pm = pkcs7(&arr);
        assert!(pkcs7_validate(&pm).is_ok());
      };
    }
    #[test]
    fn testy_4test() {
      for i in 1..=254 {
        try_pad!(0, i);
        try_pad!(1, i);
        try_pad!(254, i);
        try_pad!(255, i);
      }
    }
    #[test]
    #[should_panic(expected = "message length must be")]
    fn testy4_panic() {
      try_pad!(0, 255);
    }
  }
}
