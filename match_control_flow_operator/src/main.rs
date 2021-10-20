// Matches are exhaustive,
// the compiler is able to check if every possible
// case has been handled.
//
// The following code does not compile, for example:
//
// match Some(2) {
//  Some(x) => dbg!(x),
// } 

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  // this is the same as map, but for this example
  // it is fine
  match x {
    None => None,
    Some(i) => Some(i + 1)
  }
}

fn main() {
  assert_eq!(value_in_cents(Coin::Penny), 1);
  assert_eq!(value_in_cents(Coin::Nickel), 5);
  assert_eq!(value_in_cents(Coin::Dime), 10);
  assert_eq!(value_in_cents(Coin::Quarter), 25);

  dbg!(plus_one(None)); // None
  dbg!(plus_one(Some(1))); // Some(2)
}
