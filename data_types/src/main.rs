// Every value in Rust is of a certain data type,
// which tells Rust what kind of data is being specified
// so it knows how to work with that data.

fn main() {
  // The compiler can infer the types base on the value
  // and how use it most of the time but
  // there are situations where we will need to
  // annotate the value with its type, such as:

  let guess: u32 = "42".parse().expect("not a number!");

  // guess must be annotated with its type because
  // parse works for many types.

  println!("guess: {}", guess); // 42

  // Scalar types
  // 
  // A scaler type represents a single value.
  // Rust has four primary scalar types:
  // integers, floating-point numbers, Booleans and characters.
  //
  // Integer types
  //
  // An integer is a number without a fractional component.
  //
  // The character type
  //
  // The char type is the language's most primitive alphabetic type,
  // char literals are specified with single quotes.
   
  let z = 'z';

  println!("z: {}", z);

  // The char type is four bytes in size and represents
  // a Unicode Scalar value.

  // Compound types
  //
  // Compound types can group multiple values into one type.
  // Rust has two primitive compound types: tuples and arrays.
  //
  // The tuple type
  //
  // A tuple is a general way of grouping together a number
  // of values with a variety of types into one compound type.
  // NOTE: tuples have a fixed length.
  let tuple: (i32, f64, u8) = (500, 6.4, 1);

  println!("tuple: {:?}", tuple); // tuple: (500, 6.4, 1)

  // There are two ways to get the individual elements out of a tuple:
  // With pattern matching

  let (x, y, z) = tuple;

  println!("x: {}", x); // x: 500 
  println!("y: {}", y); // y: 6.4
  println!("z: {}", z); // z: 1

  // Directly using the a period (.) followed by the index of the
  // value we want to access.

  println!("x: {}", tuple.0); // x: 500 
  println!("y: {}", tuple.1); // y: 6.4
  println!("z: {}", tuple.2); // z: 1

  // ---
  // The array type
  //
  // Unlike a tuple, every element of an array must have the same type.
  // Arrays haved a fixed length, like tuples.
  //
  // Arrays are allocated on the stack rather than the heap(because of the fixed length).
  
  let xs = [1, 2, 3, 4, 5];

  println!("xs: {:?}", xs); // xs: [1, 2, 3, 4, 5]

  // Anottating the type
  let ys: [i32; 5] = [1, 2, 3, 4, 5];

  println!("ys: {:?}", ys); // ys: [1, 2, 3, 4, 5]

  // initializes an array of length 5 with the number 3 in
  // every position.
  let zs = [3; 5];

  println!("zs: {:?}", zs); // zs: [3, 3, 3, 3, 3]

  // Accessing array elements
  //
  // An array if a single chunk of memory allocated on the stack.
  // We can access elements of an array using indexing 

  let xs = [1, 2, 3];

  println!("{} {} {}", xs[0], xs[1], xs[2]); // 1 2 3

  // Accessing an index thats less than 0 or greater than the array length - 1
  // will cause a panic.
  //
  // xs[3] --> panics
}
