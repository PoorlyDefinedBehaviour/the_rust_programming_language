// Unit tests
//
// The purpose of unit tests is to test each unit of code in
// isolation from the rest of the code to quickly pinpoint
// where code is and isn't working as expected.
//
// The convention is to create a module named tests in each
// file to contain the test functions and annotate
// the module with cfg(test).
//
// The tests module and #[cfg(test)]
//
// The #[cfg(test)] annotation on the tests module tells Rust
// to compile and run the test code only when you run cargo test,
// which means that when we aren't running tests, the code will
// compile fast and the binary will be smaller.
//
// Integration tests
//
// In Rust, integration tests are entirely external to your library.
//
// Tt create integration tests, we need to create a tests directory.
pub fn add_two(x: i32) -> i32 {
  x + 2
}

// The attribute cfg stands for configuration and
// tells Rust that the following item should only be included
// given a certain configuration option.
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
