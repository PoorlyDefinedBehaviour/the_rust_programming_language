// Refutability: whether a pattern might fail to match
//
// Patterns come in two forms: refutable and irrefutable.
//
// Patterns that will match for any possible value passed
// are irrefutable.
//
// An example would be x in let x = 5;
//
// Patterns that can fail to match for some possible value
// are refutable.
//
// An example would be Some(x) in the expression
// if let Some(x) = EXPRESSION because if the value of
// the expression to the right of the = sign is None
// the Some(x) pattern will not match.
//
// Function parameters, let statements, and for loops can only
// accept irrefutable patterns, because the program cannot
// do anything meaningful when values don't match.
//
// The if let and while let expressions accept refutable and irrefutable
// patterns, but the compiler warns against irrefutable patterns
// because by definition they're intended to handle possible failure.
//
// Summary:
//
// Patterns that can fail will cause a compiler error
// if used in places where the compiler doesn't know what to do
// if the pattern does not match.

fn main() {
  println!("Hello, world!");
}
