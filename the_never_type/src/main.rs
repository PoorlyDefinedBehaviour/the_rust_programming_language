#[allow(unused)]
// The never type that never returns
//
// Rust has a special type named ! that's known in type theory lingo
// as the empty type because it has no values.
// Rust calls it the never type because it stands in the place
// of the return type when a function will never return.
//
// Functions that never return are called diverging functions.
fn bar() -> ! {
  panic!("oops")
}

fn main() {
  println!("Hello, world!");
}
