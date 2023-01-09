#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// see: mod problem (~line 90)
// #![feature(generic_const_exprs)]

// GenericArray and const-generics are mostly at parity. Generic Array increases time-to-compile,
// and requires at least two external crate dependencies, Typenum and GenericArray. However,
// GenericArray has been around for longer, and is slightly more flexible as the following example
// demonstrates.
mod problem {
  // The following module replicates the const-generic problem
  // documented in GenericArray: https://github.com/fizyk20/generic-array/issues/115
  // the following fails to compile unless line 5 is uncommented.
  // trait ProblemTrait {
  //   const N: usize;
  //   //
  //   // requires feature gated #![feature(generic_const_exprs)]
  //   // tracking issue: https://github.com/rust-lang/rust/issues/76560
  //   fn foo() -> [f32; Self::N];
  // }

  // the following compiles as expected, with GenericArray:
  use generic_array::{ArrayLength, GenericArray};
  trait Trait {
    type N: ArrayLength<f32>;
    fn foo() -> GenericArray<f32, Self::N>;
  }
}

mod array_wrappers {
  //! Array wrappers, demonstrating three Array-Wrapping structs:
  //! - naive `ArrayWrapper`, wrapping a Vec, allocating on the Heap at runtime.
  //! - `GenericArray` wrapper, allocating a generically sized array at compile time
  //! - Rust const-generics wrapper, also allocating a generically sized array at compile time
  use generic_array::{ArrayLength, GenericArray};

  /// Array Wrapper wrapping a Vec, allocating on the Heap at runtime.
  struct ArrayWrapper<T> {
    inner: Vec<T>,
  }

  impl<T> ArrayWrapper<T> {
    fn new() -> Self { Self { inner: Vec::new() } }

    fn get_index(&self, index: usize) -> Option<&T> { self.inner.get(index) }

    fn get_index_mut(&mut self, index: usize) -> Option<&mut T> { self.inner.get_mut(index) }

    fn len(&self) -> usize { self.inner.len() }
  }

  /// Array Wrapper wrapping a constant-sized array, allocating on the Stack at compile time.
  /// The `ArrayLength` trait is used to specify the size of the array, limited by the `Unsized`
  /// trait, restricting possible values for `T` to unsized integers.
  /// Reference: https://docs.rs/generic-array/latest/generic_array/
  struct GenArrayWrapper<T, Size: ArrayLength<T>> {
    inner: GenericArray<T, Size>,
  }

  impl<T: Default, SIZE: ArrayLength<T>> GenArrayWrapper<T, SIZE> {
    // the constructor now requires type parameters:
    fn new() -> Self { GenArrayWrapper::<T, SIZE> { inner: GenericArray::default() } }

    // The following are unchanged:
    fn get_index(&self, index: usize) -> Option<&T> { self.inner.get(index) }

    fn get_index_mut(&mut self, index: usize) -> Option<&mut T> { self.inner.get_mut(index) }

    fn len(&self) -> usize { self.inner.len() }
  }

  /// Generic Array using const generics.
  /// Unlike Generic Array, we use `usize` to restrict `T`, instead of `ArrayLength` (implicitly
  /// restricting `T` to `Unsized`), as above
  ///
  /// Using Rust-native Const Generics further requires T to be restricted by Copy.
  /// Reference: https://practice.rs/generics-traits/const-generics.html
  struct ConstArrayWrapper<T: Default + Copy, const SIZE: usize> {
    inner: [T; SIZE],
  }

  impl<T: Default + Copy, const SIZE: usize> ConstArrayWrapper<T, SIZE> {
    //
    fn new() -> Self { Self { inner: [T::default(); SIZE] } }

    // The following are unchanged:
    fn get_index(&self, index: usize) -> Option<&T> { self.inner.get(index) }

    fn get_index_mut(&mut self, index: usize) -> Option<&mut T> { self.inner.get_mut(index) }

    fn len(&self) -> usize { self.inner.len() }
  }

  #[cfg(test)]
  mod test {
    use typenum::U3;

    use super::*;
    #[test]
    fn test_wrappers() {
      let naive = ArrayWrapper::<i32>::new();
      // Generic Array requires the supplemental typenum crate for specifying array size
      let generic = GenArrayWrapper::<i32, U3>::new();

      // const-generic array does not require the use of the typenum crate.
      let const_generic = ConstArrayWrapper::<i32, 3>::new();
      // the following non-usize values will fail to compile
      // let const_generic = ConstArrayWrapper::<i32, 3.0>::new();
      // let const_generic = ConstArrayWrapper::<i32, -3>::new();
    }
  }
}
