// When we use generic type parameters, we can specify a default
// concrete type for the generic type.
//
// The syntax for specifying a default type for a generic type is
// <PlaceholderType=ConcreteType> when declaring the generic type.
//
// Example of where this is useful:
//
// Operator overloading
//
// Rust does not allow us to create our own oeprators or
// overload arbitrary operators, but we can overload
// the operations defined in std::ops by implementing
// the traits associated with the operator.

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

// Overloading the + operator for Point
impl std::ops::Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Self::Output {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

// std::ops::Add is defined as
//
// trait Add<Rhs=Self> {
//  type Output;
//
//  fn add(self, rhs: Rhs) -> Self::Output;
// }
//
// where Rhs is a generic type parameter with
// Self(the type implementing std::ops::Add as the default concrete type)

struct Milimeters(u32);
struct Meters(u32);

// If we want to add two different types,
// we can provide a concrete type that is not Self
// to std::ops::Add.
impl std::ops::Add<Meters> for Milimeters {
  type Output = Milimeters;

  fn add(self, other: Meters) -> Self::Output {
    Milimeters(self.0 + (other.0 * 1000))
  }
}
fn main() {
  assert_eq!(
    Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
    Point { x: 3, y: 3 },
  )
}
