// The compiler ensures references are always valid.
//
// Unsafe rust has two new types called raw pointers
// that are similar to references. As with references, raw pointers
// can be immutable or mutable and are written as *const T and *mut T.
// The asterisk isn't the derefence operator; it's part of the type name.
// In the context of raw pointers, immutable means that the pointer
// can't be directly assigned to after being dereferenced.
//
// Different from references and smart pointers, raw pointers:
//
// Are allowed to ignore the borrowing rules by having both immutable
// and mutable pointers or multiple mutable pointers to the same location.
//
// Aren't guaranteed to point to valid memory.
//
// Are allowed to be null.
//
// Don't implement any automatic cleanup.
//
// Why would you want to use raw pointers?
//
// To get more performance.
// To interface with another language or hardware where Rust's
// guarantees don't apply.
fn example_1() {
  let mut num = 5;

  // We can create raw pointers using safe code,
  // but cannot deference them.
  //
  // Immutable raw pointer to [num].
  let r1 = &num as *const i32;

  // Mutable raw pointer to [num].
  let r2 = &mut num as *mut i32;

  // Dereferencing a raw pointer is unsafe.
  unsafe {
    dbg!(*r1); // 5

    *r2 = 10;

    dbg!(*r2); // 10
  }
}

fn example_2() {
  let address = 0x012345usize;

  // Immutable raw pointer to a memory address.
  //
  // We can address random memory addresses using raw pointers,
  // but the memory is not guaranteed to be valid, making the
  // behaviour undefined.
  let r = address as *const i32;

  dbg!(r); // 0x0000000000012345
}

fn main() {
  example_1();
  example_2();
}
