// Implementing an unsafe trait
//
// A trait is unsafe when at least one of its methods has some
// invariant that the compiler can't verify.

// We an declare that a trait is unsafe by adding the unsafe keyword
// before trait.
unsafe trait Foo {}

// We also need to add the unsafe keyword before the impl block
// when implementing an unsafe trait.
unsafe impl Foo for i32 {}

fn main() {
  println!("Hello, world!");
}
