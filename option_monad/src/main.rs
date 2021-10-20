// Rust does not have null, it has
// the Option monad instead.
//
// The Option monad is an enum defined by the standard library,
// it looks like this:
//
// enum Option<T> {
//  None,
//  Some(T),
// }
//
// None means there is no value
// and Some(T) will hold a value of type T when
// there is a value.

fn main() {
  let some_number = Some(5);
  
  dbg!(&some_number); // Some(5)

  let some_string = Some("a string");

  dbg!(&some_string); // Some("a string")

  // variable msut be annotated with its type
  // because the compiler cannot infer the type
  // using only None 
  let absent_number: Option<i32> = None;

  dbg!(&absent_number); // None

  // Difference between null and Option
  //
  // When using Option we need to check
  // which variant we have before using the value(if there's one)
  // while won't force us to check anything most of the time.

  // we chan using Option methods to check which variant
  // it is
  if some_string.is_some() {
    // ...
  }

  // or pattern matching
  match some_string {
    None => println!("there's no string"),
    Some(s) => println!("got {}", s),
  }
}
