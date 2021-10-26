#![allow(unused)]
// How to write tests
//
// Tests are Rust functions that verify the non-test code
// is functioning in the expected manner.
//
// The bodies of test functions typically perform
// these three actions:
//
// Set up any needed data or state.
// Run the code you want to test.
// Assert the results are what you expect.
//
// The anatomy of a test function
//
// A test in Rust is a function that's annotated with the
// test attribute.
// Attributes are metadata about pieces of Rust code,
// one example is the derive attribute used with structs.
//
// To change a function into a test function, add #[test]
// on the line before the function declaration.
//
// When you run you tests with the cargo test command,
// Rust builds a test runner binary that runs the functions
// annotated with the test attribute.
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }

    Guess { value }
  }
}

fn main() {
  println!("Hello, world!");
}

// #[cfg(test)] tells Rust the the module tests
// should only compiled when the test configuration
// is active. The test configuration is active
// when we run the cargo test command.
#[cfg(test)]
mod tests {
  // Since this module is declared inside another module
  // and we want to use things that were declared
  // in the parent module, we need to import them.
  use super::*;

  // #[test] indicates that this function
  // is a test function.
  // We also could have functions that are not test functions
  // inside this module.
  #[test]
  fn it_works() {
    // Panics if values are not equal,
    // making the test fail.
    assert_eq!(2 + 2, 4)
  }

  #[test]
  fn larger_rectangle_can_hold_smaller_rectangle() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_rectangle_cannot_hold_larger_rectangle() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    assert!(!smaller.can_hold(&larger));
  }

  // Checking for panics with should_panic
  //
  // In addition to checking that our code returns the
  // values we expect, it's also important to check
  // that our code handles error conditions as we expect.
  //
  // We can ensure that our code panics in expected conditions
  // using the should_panic attribute. This attribute makes
  // the test pass if the code inside the test panics.
  //
  // NOTE: Tests that use the should_panic attribute
  // are more imprecisve because the test will pass
  // if the code panics for any reason.
  //
  // In the Guess case, would be better to change the
  //
  // enum GuessError {
  //   OutOfBounds(i32)
  // }

  // new function signature to fn new() -> Result<Guess, GuessError>
  // instead of fn new() -> Guess and panicking if
  // the guess is not valid.
  #[test]
  #[should_panic]
  fn guess_should_panic_when_number_is_less_than_1() {
    let _ = Guess::new(0);
  }

  #[test]
  #[should_panic]
  fn guess_should_panic_when_number_is_less_greater_than_100() {
    let _ = Guess::new(101);
  }

  // We also can assert that the panic message
  // is what we expect
  #[test]
  #[should_panic(expected = "Guess value must be between 1 and 100, got 101.")]
  fn guess_should_panic_when_number_is_less_greater_than_100_with_message() {
    let _ = Guess::new(101);
  }

  // We can return Result<T, E> from a test.
  // It is useful when we have functions
  // that return a Result inside the test function
  // because it allows us to use the ? operator.
  #[test]
  fn result_test() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }
}
