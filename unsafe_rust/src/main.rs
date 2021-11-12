// Code in rust normally has safety guarantees enforced
// at compile time. However, Rust has a second language hidden
// inside it that doesn't enforce these memory safety guarantees.
// It's called unsafe Rust.
//
// Unsafe Rust exists because, by nature, static analysis is conservative.
// When the compiler tries to determine whether or not code
// upholds the guarantees, it's better for it to reject some valid
// programs rather than accept some invalid programs.
//
// TLDR: The compiler can't always tell if a program is safe or correct,
// so when in doubt, it will reject the program.
// We can use unsafe Rust when we know the program is correct but
// the compiler can't tell.
//
// Another reason unsafe Rust exists is because the underlying
// computer hardware is unherently unsafe. If Rust didn't let you
// do unsafe operations, you couldn't do certain tasks.
//
// Unsafe superpowers
//
// To switch to unsafe Rust, use the unsafe keyword and then
// start a new block that holds the unsafe code.
//
// Things you can do in unsafe Rust but not in safe Rust:
//
// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions
//
// Note that unsafe does't turn off the borrow checker or disable
// any other of Rust's safety checks. The unsafe keyword only gives
// you access to these five features that are then not checked
// by the compiler for memory safety.
//
// To isolate unsafe code as much as possible, it's best to enclose
// unsafe code within a safe abstraction and provide a safe API.
//
// Parts of the standard library are implemented as safe abstractions
// over unsafe code that has been audited.
//
// Wrapping unsafe code in a safe abstraction prevents uses of unsafe
// from leaking out into all the places that you or your users
// might want to use the functionality implemented with unsafe code.

fn main() {
  println!("Hello, world!");
}
