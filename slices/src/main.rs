// Another data type that does not have ownership is the slice.
// Slices let you reference a contiguous sequence of 
// elements in a collection rather than the whole collection.

// Returns the index of the first word in the string
// A string slice is a reference to a part of a String, and it
// looks like this:
// let s = String::from("hello world");
// let hello = &s[0..5];
// let world = &s[6..1];
// &str signifies string slice.
fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  } 

  &s[..]
}

// Same as first_word but takes a &str
// which works for &str and &String instead of
// just &String
fn first_word_2(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  } 

  &s[..]
}

fn main() {
  let s = String::from("hello world");

  let word = first_word(&s);
  
  dbg!(&word); // hello

  // compile error: cannot borrow s as mutable
  // because it is also borrowed as immutable.
  // word is a reference to a part of s,
  // we cannot clear it because it would make word invalid.
  //
  // s.clear();

  dbg!(&word); 

  // Strings literals are slices.
  // String literals are stored inside the binary,
  // s is a slice pointing to that specific point in the binary.
  // This is also why string literals are immutable; 
  // &str is an immutable reference
  let s: &str = "Hello, world!";
  dbg!(s);

  dbg!(first_word_2(s)); // Hello,
  dbg!(first_word_2(&String::from("Hello, world!"))); // Hello,

  // Other slices
  // There's a more general slice type:
  let xs = [1, 2, 3, 4, 5];

  let slice: &[i32]= &xs[1..3];

  assert_eq!(slice, &[2, 3]);
}
