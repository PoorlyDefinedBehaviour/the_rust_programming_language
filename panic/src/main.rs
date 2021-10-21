// When the panic! macro executes, the program will
// print a failure message, unwind and clean up the stack,
// and then quit.
//
// By default when a panic occurs, Rust walks back up
// the stack and cleans up the data from each
// function it encounters.
//
// Unwinding is a lot of work, the alternative is
// to immediately abort, which ends the program without
// cleaning up. The memory the program was using
// will then need to be cleaned up by the operating system.
//
// We can choose to unwind the stack or not by
// changing Cargo.toml.
//
// Add this to Cargo.toml:
//
// [profile.release]
// panic = 'abort'
//
// to not unwind the stack on panics in release mode.
fn main() {
  panic!("crash and burn");
}
