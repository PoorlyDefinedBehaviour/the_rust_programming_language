// Rust is an expression-based language
// where function bodies are made up of a series of statements
// optionally ending in an expression.
//
// Statements are instructions that perform some action
// and do not return a value.
//
// Expressions evaluate to a resulting value.

// Function definitions are statements. 
//
// Function signatures must declared the type
// of each parameter.
fn f(x: i32) {
  println!("the value of x is: {}", x);
}

// We can omit the return keyword
// in the last expression of the block
// to implictly return it 
fn five() -> i32 {

  5
}

fn main() {
  f(10);

  // Creating a variable and assigning a value to it
  // with the let keyword is a statement
  let x = 3;

  dbg!(x);
  
  // Since statements do not produce values, we can't not assign
  // the result of a let statement to a variable 
  //
  // let x = (let y = 6) --> does not compile

  let x = 5;

  dbg!(x); // x = 5

  let y = {
    let x = 3;
    x + 1
  }; 

  dbg!(y); // y = 4

  dbg!(five());
}
