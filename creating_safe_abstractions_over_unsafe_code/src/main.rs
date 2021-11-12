#![allow(unused)]
// Creating a safe abstraction over unsafe code
//
// Wrapping unsafe code in a safe function is a common abstraction.
//
// Let's look at the split_at_mut method which is defined in the
// standard library.
//
// This function cannot be implemented using safe Rust because
// it returns two mutable references to the same slice and
// we cannot have two mutable references to a value using
// safe Rust.
use core::slice;

fn split_at_mut<T>(xs: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
  let len = xs.len();
  let ptr = xs.as_mut_ptr();

  debug_assert!(mid <= len);

  unsafe {
    (
      // Create a slice up to the middle of [slice].
      slice::from_raw_parts_mut(ptr, mid),
      // Create a slice starting from [mid] of [slice] up to end of [slice].
      slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )
  }
}

fn main() {
  println!("Hello, world!");
}
