// Macros
//
// The term macro refers to a family of features in Rust:
// declarative macros with macro_rules! and three kinds of
// procedural macros:
//
// - Custom #[derive] macros that specify code added with the derive
// attribute used on structs and enums
// - Attribute-like macros that define custom attributes usable on any item
// - Function-like macros that look like function calls but operate
// on the tokens specified as their argument
//
// The difference between macros and functions
//
// Fundamentally, macros are a way of writing code that writes other code,
// which is known as metaprogramming.
//
// Metaprogramming is useful for reducing the amount of code
// you have to write and maintain, which is also one of the roles of functions.
// However, macros have some additional powers that functions don't.
//
// A function signature must declare the number and type of parameters
// the function has.
// Macros, on the other hand, can take a variable numnber of parameters:
// we can call println!("hello") with one argument or println!("hello {}", name) with two arguments.
// Also macros are expanded before the compiler interprets the meaning of the code,
// so a macro can, for example, implement a trait on a given type.
// A function can't, because it gets called at runtime and a trait needs to be
// implemented at compile time.
//
// Declarative macros with macro_rules! for general metaprogramming
//
// macro_rules! macros allow you to write something similar to a Rust match expression.
//
// It compares a value to patterns that are associated with particular code:
// in this situation, the value is the literal Rust source code passed
// to the macro; the patterns are compared with the structure
// of that source code; and the code associated with each pattern,
// when matched, replaces the code passed to the macro. This all happens
// during compilation.

// Example: simplified vec! macro
//
// The #[macro_export] annotation indicates that this macro
// should be made available whenever the crate in which the macro
// is defined is brought into scope.
#[macro_export]
macro_rules! vec {
  ($($x: expr),*) => {
    {
      let mut temp_vec = Vec::new();
      $(temp_vec.push($x))*;
      temp_vec
    }
  };
}

fn main() {
  println!("Hello, world!");
}
