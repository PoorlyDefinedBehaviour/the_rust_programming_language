// Matching literals
//
// We can match patterns against literals directly.
fn matching_literals() {
  let x = 1;

  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
  }
}

// Matching named variables
//
// Named variable are irefutable patterns that match any value.
//
// match starts a new scope, variables declared as part of a pattern
// inside the match expression will shadow those with the same name
// outside match construct.
//
// NOTE:
// Can we match like elixir using ^x?
// I think we cant, but we can use match guard conditionals.
fn matching_named_variables() {
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("got 50"),
    Some(y) => println!("y = {:?}", y), // y = 5
    _ => println!("default case, x = {:?}", x),
  }

  println!("at the end, x = {:?}, y = {:?}", x, y);
  // x = Some(5)
  // y = 10
}

// Multiple patterns
//
// In match expressions, you can match multiple patterns
// using the | syntax, which menas or.
fn multiple_patterns() {
  let x = 1;

  match x {
    1 | 2 => println!("one or two"), // will match
    3 => println!("three"),
    _ => println!("anything"),
  }
}

// Matching ranges
//
// The ..= syntax allows us to match an inclusive range of values.
fn matching_ranges() {
  let x = 5;

  match x {
    // Will match 1, 2, 3, 4, 5.
    //
    // Ranges are only allowed with numeric values or char values.
    1..=5 => println!("one through five"), // will match
    _ => println!("something else"),
  }
}

// Destructuring to break apart values
//
// We can use patterns to destructure structs, enums, tuples,
// and references to use different parts of these values.
//
// Destructuring structs
struct Point {
  x: i32,
  y: i32,
}

fn destructuring_structs() {
  let p = Point { x: 0, y: 7 };

  // Renaming is optional.
  let Point { x: a, y: b } = p;

  assert_eq!(0, a);

  assert_eq!(7, b);

  let Point { x, y } = p;

  assert_eq!(0, x);

  assert_eq!(7, y);

  match p {
    // Matches any point if y is 0.
    Point { x, y: 0 } => println!("x={}, y={}", x, y),
    // Mathces any point if x is 0.
    Point { x: 0, y } => println!("x={}, y={}", x, y),
    // Matches any point.
    Point { x, y } => println!("x={}, y={}", x, y),
  }
}

fn main() {
  matching_literals();
  matching_named_variables();
  multiple_patterns();
  matching_ranges();
  destructuring_structs();
}
