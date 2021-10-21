#[derive(Debug)]
enum Maybe<T> {
  Some(T),
  None,
}

#[derive(Debug)]
enum Either<L, R> {
  Left(L),
  Right(R),
}
use Either::*;

fn foo(x: i32) -> Either<String, bool> {
  if x < 10 {
    Left(String::from("hello"))
  } else {
    Right(true)
  }
}

fn max<T: PartialOrd>(list: &[T]) -> Maybe<&T> {
  if list.is_empty() {
    return Maybe::None;
  }

  let mut greatest = &list[0];

  for element in list {
    if element > greatest {
      greatest = element;
    }
  }

  Maybe::Some(greatest)
}

#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

// Methods available for Point of any type
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}

// Methods available for Point of f32
// We can implement methods that will be available
// only for specific types.
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

fn main() {
  let numbers = vec![34, 50, 25, 100, 65];

  dbg!(max(&numbers)); // Some(100)

  // int
  dbg!(Point { x: 5, y: 10 });

  // float
  dbg!(Point { x: 1.0, y: 4.0 });

  dbg!(Point { x: 1.0, y: 4.0 }.distance_from_origin());

  // Does not compile because distance_from_origin
  // is only implemented for Point<f32> and we have
  // a Point<i32>
  // dbg!(Point { x: 1, y: 4 }.distance_from_origin());

  dbg!(Point { x: 1.0, y: 4.0 }.x());

  dbg!(Point { x: 1.0, y: 4.0 }.x());

  dbg!(foo(5)); // Left("hello")

  dbg!(foo(30)); // Right(true)
}
