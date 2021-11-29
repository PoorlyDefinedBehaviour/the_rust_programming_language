// Using supertraits to require one trait's functionality within another trait
//
// Sometimes, we might need to one trait to use
// another trait's functionality. In this case, we need
// to rely on the dependent trait also being implemented.
// The trait we rely on is a supertrait of the trait
// we are implementing.

// Any type that implements OutlinePrint must also implement
// std::fmt::Display because we use the to_string method.
trait OutlinePrint: std::fmt::Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();

    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

struct Point {
  x: i32,
  y: i32,
}

impl OutlinePrint for Point {}

// Must be implemented because OutlinePrint requires it.
impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

fn main() {
  let point = Point { x: 2, y: 2 };
  // **********
  // *        *
  // * (2, 2) *
  // *        *
  // **********
  point.outline_print();
}
