// Calling an unsafe function or method
//
// The second type of operation that requies an unsafe block is calls
// to unsafe functions(the first is dereferencing a raw pointer).
//
// Unsafe functions and methods look the same as normal functions and methods,
// except that they use the unsafe keyword which indicates
// that the function has requirements we need to uphold when we
// call this function, because Rust can't guarantee them.
unsafe fn dangerous() {
  println!("dangerous");
}

fn example_1() {
  // We need to use unsafe blocks to call unsafe functions or methods.
  // Calling unsafe functions or methods without unsafe blocks
  // is a compile error.
  unsafe {
    dangerous();
  }
}

struct S;

impl S {
  unsafe fn do_something(&self) {
    println!("S.do_something()");
  }
}

fn example_2() {
  let s = S;

  unsafe {
    s.do_something();
  }
}

fn main() {
  example_1();
  example_2();
}
