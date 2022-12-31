#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::marker::PhantomData;

// use anyhow::Result;
use error::MyError;

mod error;
#[cfg(test)] mod tests;
mod utils;
use anyhow::anyhow;

// thiserror allows us to ergonomically call into(), to cast the anyhow::Error as MyError
fn example_error() -> Result<(), MyError> { Err(anyhow!("Example error").into()) }
fn example_success() -> Result<(), MyError> { Ok(()) }

mod type_level {
  use std::marker::PhantomData;

  use tylift::tylift;

  // tylift is a convenience macro, reifying enum variants to their own types.
  // https://docs.rs/tylift/latest/tylift/attr.tylift.html
  //
  /// MyState is a type-level enum.
  ///
  /// This is mostly equivalent to:
  /// use __kind_MyState::*;
  /// mod __kind_MyState {
  ///   use super::*;
  ///   pub struct Mutable(::core::marker::PhantomData<()>);
  ///   impl MyState for Mutable {}
  ///   pub struct Immutable(::core::marker::PhantomData<()>);
  ///   impl MyState for Immutable {}
  /// }
  #[tylift]
  enum MyState {
    Mutable,
    Immutable,
  }

  /// A sample state machine parameterized over a custom mutable/immutable state enum.
  struct MyStateMachine<S>
  where S: MyState {
    y:       i32,
    // A common use of PhantomData is to parameterize state by a type that is not used in the
    // type's definition. This is exactly what we want here.
    phantom: PhantomData<S>,
  }

  /// Convenience alias for MyStateMachine<S: MyState>, so that we don't have to write out the full
  /// type in API calls.
  type MutableStateMachine = MyStateMachine<Mutable>;

  // methods available in either state
  impl<S: MyState> MyStateMachine<S> {
    fn new_mutable() -> MyStateMachine<Mutable> { MyStateMachine { y: 0, phantom: PhantomData } }

    fn get_y(&self) -> i32 { self.y }
  }

  // methods available when state is Mutable
  impl MyStateMachine<Mutable> {
    fn increment(&mut self) { self.y += 1; }

    fn freeze(self) -> MyStateMachine<Immutable> {
      MyStateMachine { y: self.y, phantom: PhantomData }
    }

    fn reset(mut self) -> MyStateMachine<Mutable> {
      self.y = 0;
      MyStateMachine { y: self.y, phantom: PhantomData }
    }
  }
  impl MyStateMachine<Immutable> {
    fn increment(&self) { println!("can't increment! State is immutable.") }
  }

  #[cfg(test)]
  mod test {
    use super::*;

    #[test]
    fn test_state_machine() {
      let mut m = MutableStateMachine::new_mutable();
      assert_eq!(m.get_y(), 0);
      m.increment();
      assert_eq!(m.get_y(), 1);
      m.increment();
      assert_eq!(m.get_y(), 2);
      let m = m.freeze();
      assert_eq!(m.get_y(), 2);
      m.increment();
      assert_eq!(m.get_y(), 2);
      // let m = i.reset(); // that fuzzy feeling when your code, by design, does not compile ✅
    }
  }
}

mod compile_with_malicious {
  /// If compiled with the malicous feature, return the second input.
  macro_rules! evil {
    ($var:ident) => {
      if cfg!(feature = "mal") {
        "not goodboi".to_string()
      } else {
        $var
      }
      // equiv to:
      // #[cfg(feature = "mal")]
      // println!("a malicious client is trying to obtain the biscuit, with corrupt={:?}",
      // $corrupt); #[cfg(feature = "mal")]
      // let $var = $corrupt;
    };
  }

  /// If compiled with without the malicious feature, goodboi is good.
  /// Otherwise, let the user decide what goodboi is.
  /// This is characteristic of what a malicious source modification may be able to achieve.
  /// For instance, in a client-server model, the client may modify their source code to request
  /// data from the server that they are not authorized to obtain.
  ///
  /// The server could use this pattern to test that malicious clients are not able to obtain
  /// more than they should be able to.
  ///
  /// This pattern can be implemented in a more sophisticated way. If it is possible to enumerate
  /// all ways that a client can abuse the server, then not_goodboi should be an enum of all
  /// misbehavior types.
  pub fn goodboi() -> String {
    let out = "goodboi".to_string();
    evil!(out)
  }

  #[cfg(test)]
  mod test {
    use tracing::info;

    use super::*;

    // test with `cargo test --all-features` or `cargo test --features malicious`
    #[test]
    fn test_goodboi() {
      let output = goodboi();

      if cfg!(feature = "mal") {
        assert_eq!(output, "not goodboi");
      } else {
        assert_eq!(output, "goodboi");
      }
    }
  }
}

pub mod secret_data_branching {
  //! Observed:
  //!
  //! left                    time:   [1.9281 ns 1.9461 ns 1.9740 ns]                  
  //! Found 7 outliers among 100 measurements (7.00%)
  //!   2 (2.00%) high mild
  //!   5 (5.00%) high severe
  //!
  //! right                   time:   [1.6009 ns 1.6049 ns 1.6098 ns]                   
  //! Found 9 outliers among 100 measurements (9.00%)
  //!   4 (4.00%) high mild
  //!   5 (5.00%) high severe
  //!
  //! vleft                   time:   [361.18 ns 362.61 ns 364.27 ns]                  
  //! Found 8 outliers among 100 measurements (8.00%)
  //!   5 (5.00%) high mild
  //!   3 (3.00%) high severe
  //!
  //! vright                  time:   [353.31 ns 354.01 ns 354.80 ns]                   
  //! Found 6 outliers among 100 measurements (6.00%)
  //!   3 (3.00%) high mild
  //!   3 (3.00%) high severe
  //!
  //! This corresponds with expectation; the compiler can short circuit on shorter values in the
  //! vector. Comparing with the long SECRET value takes longer in the left paths. This
  //! demonstrates that information can be inferred from branching on the long SECRET value.

  use once_cell::sync::Lazy;

  pub const SECRET: u64 = 123456789012345678;
  // need to use a Lazy to initialize a static with a non-const value
  pub static SECRET_VEC: Lazy<Vec<u64>> = Lazy::new(|| vec![SECRET; 100]);

  /// take the left branch if lr is true, otherwise take the right branch

  pub fn branches_on_secret(val: u64) {
    let mut left_counter = 0;
    let mut right_counter = 0;
    for i in 0..=999 {
      if val == SECRET {
        left_counter += 1;
      } else {
        right_counter += 1;
      }
    }
    // println!("left_counter={left_counter}, right_counter={right_counter}");
  }

  pub fn branches_on_secret_vec(val: Vec<u64>) {
    let mut left_counter = 0;
    let mut right_counter = 0;
    for i in 0..=999 {
      if val == *SECRET_VEC {
        left_counter += 1;
      } else {
        right_counter += 1;
      }
    }
    // println!("left_counter={left_counter}, right_counter={right_counter}");
  }
}
