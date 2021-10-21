#![allow(unused)]
// What is a String
//
// Rust has only one string type in the core language,
// which if the string slice str.
//
// The String type which is provided by standard library
// rather than coded into the code language, is glowable,
// mutable, owned, UTF-8 encoded string type.
//
// Both String and string slices are UTF-8 encoded.

fn main() {
  let s = String::new();

  dbg!(&s); // ""

  let s: String = "initial contents".to_string();

  dbg!(&s); // "initial contents"

  let s = String::from("initial contents");

  dbg!(&s); // "initial contents"

  // Updating a String
  // A String can grow in size and its contents can change.
  let mut s = String::from("foo");
  s.push_str("bar");

  dbg!(&s); // "foobar"

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  // + signature for adding strings:
  // fn add(self, s: &str) -> String
  let s3 = s1 + &s2;

  dbg!(&s3); // "Hello, world!"

  // For more complicated String formatting, we can use format!
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{}-{}-{}", s1, s2, s3);

  dbg!(&s); // "tic-tac-toe"

  // Indexing into Strings
  // We cannot index into Strings like in other languages,
  // for example, this will not compile:
  // let s1 = String::from("hello");
  // let h = s1[0]; --> the type `String` cannot be indexed by `{integer}`
  // Internal representation
  // A String is a wrapper over a Vec<u8>.
  // hello.len() is 4 in this case which means
  // the Vec<u8> used to store the string "Hola" is 4 bytes long.
  // Each of these letters takes 1 byte when encoded in UTF-8.
  let hello = String::from("Hola");

  // But this time hello.len() is 24 even though it doens't have
  // 24 characters.
  // 24 is the number of bytes that takes to encode this string
  // in UTF-8.
  let hello = String::from("Здравствуйте");

  // Why we cant index into Strings
  // The reason why we cannot index into Strings,
  // is that the index will not always correspond to
  // a single character because Strings are encoded
  // in UTF-8
  //
  // Another reason is that indexing operations
  // are expected to take constant time O(1) and
  // that cannot be guaranteed with the String type.

  // Iterating of strings
  for c in "नमस्ते".chars() {
    println!("{}", c);
  }
}
