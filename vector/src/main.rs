// Vectors allow you to store more than one value in a single
// data structure that puts all the values next to each
// other in memory.
//
// Vectors can only store values of the same type.

fn main() {
  // Type annotation is needed because we haven't
  // added any values to the vector and because of that
  // the compiler cannot infer its type.
  let v: Vec<i32> = Vec::new();

  dbg!(&v); // []

  // vec! is a macro that creates a new Vec
  // with the elements inside of it.
  // v: Vec<i32>
  let v = vec![1, 2, 3];

  dbg!(&v); // [1, 2, 3]

  // Updating a vector
  // As usual, it can only be mutated if we
  // make it mutable by adding the mut keyword
  // v: Vec<i32> because we are pushing i32's into it.
  let mut v = Vec::new();

  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);

  dbg!(&v); // [5, 6, 7, 8]

  // Dropping a vector drops its elements
  // Like any other struct, a vector is free when it
  // goes out of scope.
  // Its elements will also be dropped if needed.
  {
    let v = vec![1, 2, 3, 4];
    dbg!(&v); // [1, 2, 3, 4]
  } // v goes out of scope and is freed here

  // Reading elements
  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];

  dbg!(&third); // 3

  match v.get(2) {
    Some(third) => println!("the third element is: {}", third), // the third element is: 3
    None => println!("there is no third element"),
  }

  let v = vec![100, 32, 57];

  for i in &v {
    println!("{}", i);
  }

  let mut v = vec![100, 32, 57];

  for i in &mut v {
    *i += 50;
  }

  dbg!(&v); // [150, 82, 107]
}
