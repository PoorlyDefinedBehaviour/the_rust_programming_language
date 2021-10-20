// Constants are compile time values,
// we must always annotate them with their types.
//
// Constants can be declared in any scope.
const MAX_POINTS: u32 = 100_000;

fn main() {
  println!("max points: {}",MAX_POINTS);

  mutability();

  shadowing();
}

fn mutability() {
  // Variables are immutable by default,
  // we can use the mut keyword to make a variable mutable
  let mut x = 5;

  println!("the value of x is: {}", x);

  // Would fail at compile time if we didn't use the mut keyword
  x = 6;

  println!("the value of x is: {}", x);
}

// We can declare a new variable with the same
// name as a previous variable. 
// The first variable ios shadowed by the second,
// which means that the second variable`s value if what
// appears when the variable is used.
fn shadowing() {
  let x = 5;

  let x = x + 1; // 6

  let x = x * 2; // 12

  println!("the value of x is: {}", x);

  // Shadowing is not eh same as marking a variable as mut,
  // because we'll get a compile-time error if
  // we accidentally try to reassign to this variable.
  //
  // let spaces = " ";
  // let spaces = spaces.len() --> compile-time error

  // Another difference between mut and shadowing is that
  // because we create a new variable when we use the
  // let keyword again, we can change the type of the
  // value but reuse the same name
  let y = 1;

  println!("y: {}", y);

  let y = "hello";

  println!("y: {}", y);
}
