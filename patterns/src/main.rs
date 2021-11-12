// Patterns and matching
//
// Patterns are a special syntax in Rust for matching against
// the structure of types, both complex and simple.
// Using patterns in conjunction with match expressions and other
// constructs gives you more control over a program's control flow.
// A pattern consists of some combination of the following:
//
// Literals
// Destructured arrays, enums, structs, or tuples
// Variables
// Wildcards
// Placeholders
//
// All the places patterns can be used
//
// match Arms
//
// We use patterns in the arms of match expressions.
// Formally, match expressions are defined as the keyword match,
// a value to match on, and one or more match arms
// that consist of a pattern and an expression to run
// if the value matches that arm's pattern:
//
// match VALUE {
//  PATTERN => EXPRESSION,
//  PATTERN => EXPRESSION,
//  PATTERN => EXPRESSION,
// }
//
// match expressions need to be exhaustive in the sense
// that all possibilities for the value in the match expression
// must be accounted for.
enum Tree<T> {
  Node(Box<Tree<T>>, T, Box<Tree<T>>),
  Leaf,
}

impl<T> Tree<T> {
  pub fn height(&self) -> usize {
    use Tree::*;

    match self {
      Leaf => 0,
      Node(left, _, right) => 1 + std::cmp::max(left.height(), right.height()),
    }
  }
}

fn match_pattern() {
  let empty_tree = Tree::<i32>::Leaf;

  dbg!(empty_tree.height()); // 0

  let tree = Tree::Node(
    Box::new(Tree::Node(Box::new(Tree::Leaf), 1, Box::new(Tree::Leaf))),
    2,
    Box::new(Tree::Node(
      Box::new(Tree::Leaf),
      3,
      Box::new(Tree::Node(Box::new(Tree::Leaf), 4, Box::new(Tree::Leaf))),
    )),
  );

  dbg!(tree.height()); // 3
}

// Conditional if let expressions
//
// It's possible to mix and match if let, else if, and else
// if let expressions.
//
// One downside of using if let expressions is that the
// compiler doesn't check exhaustiveness, whereas
// with match expressions it does.
fn if_let_expressions() {
  let favorite_color: Option<&str> = None;

  let is_tuesday = false;

  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("favoriet color is {}", color);
  } else if is_tuesday {
    println!("it's tuesday");
    // Using if let expression in else if branch.
  } else if let Ok(age) = age {
    if age > 30 {
      println!("age over 30");
    } else {
      println!("age not over 30");
    }
  } else {
    println!("didn't match anything");
  }
}

// while let Conditional loops
//
// while let allows a while loop to run for as long a
// pattern continues to match.
fn while_let() {
  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  // Vec::pop() on an empty Vec returns None.
  while let Some(top) = stack.pop() {
    println!("{}", top);
    // 3
    // 2
    // 1
  }
}

// for loops
//
// We can use patterns in after the for keyword.
fn for_loops() {
  let v = vec!['a', 'b', 'c'];

  // Iterator<T>::enumerate zips the values with its indices.
  //
  // vec!['a', 'b', 'c'].iter().enumerate().collect::<Vec<_>>() == vec![(0, 'a'), (1, 'b'), (2, 'c')]
  for (index, value) in v.iter().enumerate() {
    println!("{} at index {}", value, index);
    // 'a' is at index 0
    // 'b' is at index 1
    // 'c' is at index 2
  }
}

// let statements use patterns(elixir vibes).
fn let_statemtens_use_patterns() {
  // let PATTERN = EXPRESSION;
  let x = 5;

  dbg!(x); // 5

  let (x, y, z) = (1, 2, 3);

  dbg!(x, y, z); // 1, 2, 3

  // Just like elixir, if the number of elements in the pattern
  // doesn't match the number of elements in a tuple, the
  // overall type won't match and we'll get a compiler error.
  // let (x, y) = (1, 2, 3); ~ERR
}

// Function parameters
//
// Function parameters can also be patterns.

// [x] is a pattern just like in let expressions.
fn foo(x: i32) {
  println!("x = {}", x);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
  println!("current location: ({}, {})", x, y);
}

fn main() {
  match_pattern();
  if_let_expressions();
  while_let();
  for_loops();
  let_statemtens_use_patterns();
  foo(1);
  print_coordinates(&(1, 2));
}
