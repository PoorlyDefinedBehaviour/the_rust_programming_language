// cargo test compiles the code in test mode
// and runs the resulting binary.
//
// The default behaviour of the binary produced by cargo test
// is to run all the tests in parallel and capture output
// generated during tests.
//
// Since tests run in parallel, they should not
// depend on shared state.
//
// We can run tests in sequence by providing the --test-threads=<int> flag.
//
// cargo test -- --test-threads=1
//
// cargo test captures all outputs when tests pass by default,
// we can tell it to show the output for passing tests
// with the --show-output flag.
//
// cargo test -- --show-output
//
// We can run specific tests instead of all tests by
// passing the test name to cargo test.
//
// cargo test my_test_function_name
//
// Every test that matches the name passed to cargo test
// will be ran.
//
// If we had these two tests:
//
// #[test]
// fn a_test_1() {}
//
// #[test]
// fn a_test_2 {}
//
// and ran cargo test a_test
//
// both tests would be run.
//
//
// Ignoring some tests unless specifically requests
//
// Some tests can be expensive to execute, so we might want to
// exclude them from most runs of cargo test.
// We can do that with the ignore attribute.
//
// #[test]
// #ignore
// fn expensive_test {}
//
// we can run ignored tests with the --ignored flag:
//
// cargo test -- --ignored

fn main() {
  println!("Hello, world!");
}
