// Functions that don't take references take
// ownership of its arguments
fn calculate_length_1(s: String) ->usize {
  s.len()
}

// Functions that don't wont to take ownership of its
// arguments can take references.
// This is called borrowing.
// Since the function does not own the value,
// the value won't be dropped when the reference
// goes out of scope.
// NOTE: references are immutable by default.
fn calculate_length_2(s: &String) ->usize {
  s.len()
}

// We need to add the mut keyword if we want to modify the value.
fn calculate_length_3(s: &mut String) ->usize {
  s.push_str("!!!");
  s.len()
}

fn main() {
  let s = String::from("hello world");

  dbg!(calculate_length_1(s)); // 11
  // dbg!(calculate_length_1(s)); --> compiler error: use of moved value

  let s = String::from("hello world");

  dbg!(calculate_length_2(&s)); // 11
  dbg!(calculate_length_2(&s)); // 11

  let mut s = String::from("hello world");

  dbg!(&calculate_length_3(&mut s)); // 14
  dbg!(&s); // hello world!!!

  // About mutable references 
  // You can only have one mutable reference to a particular
  // piece of data in a particular scope.
  //
  // The benefit of having this restriction is that Rust can
  // prevent data races at compile time.
  // A data race is similar to a race condition and happens
  // when these three behaviors occur:
  //
  // Two or more pointers access the same data at the same time.
  // At least one of the pointers is being used to write to the data.
  // There's no mechanism being used to synchronize access to the data.
  let mut s = String::from("hello");

  // Summary on borrowing
  //
  // Multiple & --> Ok, since data does not change, multiple
  // actors can read it at the same time
  //
  // Multiple &mut --> error because data may be modified at the same
  // time by multiple actors
  //
  // & and &mut at the same time --> error because data may change
  // an affect the actors that's reading through the immutable reference
  // in unexpected ways.

  let r1 = &mut s; // ok
  // compile error: cannot borrow s as mutable more than once
  // let r2 = &mut s;

  dbg!(r1); // hello
}
