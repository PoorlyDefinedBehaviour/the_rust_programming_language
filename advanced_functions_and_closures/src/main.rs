#![allow(unused)]
// Advanced functions and closures
//
// Function pointers
//
// We can pass regular functions to functions.
// Functions coerce to the type fn which is called
// a function pointer.
//
// Function pointers implement all three of the closure traits:
// Fn, FnMut and FnOnce, so we cann always pass a function pointer
// as argument for a function that expects a closure.
fn add_one(x: i32) -> i32 {
  x + 1
}

// [f] is a function pointer that points to a function of type
// i32 -> i32.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

// We have another useful pattern that exploits an implementation detail
// of tuple structs and tuple-struct enum variants.
// These types use () as initializer syntax, which looks like a
// function call. The initializers are actually implemented as functions
// returning an instance that's constructed from their arguments.
// We can use these initializder functions as function pointers that
// implement the closure traits, which means we can specify the initializer
// functions as arguments for methods that take closures, like so:
enum Status {
  Value(u32),
  Stop,
}

fn example_enum_initializer() {
  //                          Status::Value is a function pointer.
  let xs: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// Returning closures
//
// Closures are represented by traits, which means you can't return closures
// directly. In most caes where you might want to return a trait,
// you can instead use the concrete type that implements the trait
// as the return value of the function. But you can't do that with
// closures because they don't have a concrete type that is returnable;
// you're not allowed to use the function pointers fn as return type, for example.
//
// For example, this code does not compile:
//
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//   |x| x + 1
// }
//
// It does not compile because the size of the closure
// is not known at compile time, it is not sized.
// To work around it, we need to make it sized.
//
// Box is a pointer type, so we can put a closure inside of it
// to make it Sized.
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

fn main() {
  println!("Hello, world!");
}
