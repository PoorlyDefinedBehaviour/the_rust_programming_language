// The tests directory
//
// We create a tests directory at the top level of our project directory.
// Cargo knows to look for integration test files in this directory.
// We can have as many test files as we want in this directory,
// and Cargo will compile each of the files as an individual crate.
use test_organization::adder;

#[test]
fn it_adds_two() {
  assert_eq!(4, adder::add_two(2));
}
