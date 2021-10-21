// The result monad is defined as an enum:
//
// enum Result<T, E> {
//  Ok(T),
//  Err(E),
// }
//
// It is meant to be used for recoverable errors,
// a function that tries to open a file should return
// Result because the file may not exist.
use std::fs::File;

fn main() {
  match File::open("i_dont_exist.txt") {
    Ok(_) => println!("got the file open"),
    Err(error) => println!("couldn't open file: {}", error),
  }
}
