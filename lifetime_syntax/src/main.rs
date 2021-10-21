#![allow(unused)]
// Every reference in Rust has a lifetime,
// which is the scope for which that reference is valid.
// Most of the time, lifetimes are implicit and inferred.

// Prevent dangling references with lifetimes
//
// The aim of lifetimes is to prevent dangling references,
// which cause a program to reference data other than the data
// it's intended to reference.

fn lifetime_error_example_1() {
  // {
  //   We can declare variables without initial values
  //   but can't use them before they are initialized.
  //   let r;
  //
  //   {
  //     let x = 5;
  //     This does not compile because r lives longer and x
  //     and r is used after x goes out of scope
  //     r = &x;
  //   }
  //
  //   println!("r: {}", r);
  // }
}

// The borrow checker
//
// The Rust compiler has a borrow checker that compares scopes
// to determine whether all borrows are valid.
//
// The summary is this:
//
// A refence must live at least as long as it is in use.
//
// If we have a variable x that is a reference to a variable y.
// y must live at least as long as the last time x is used.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// Lifetime elision
//
// This codecompiles without lifetime annotations
// for historical reasons.
//
// Earlier versions of Rust required the developers
// to always inform the lifetime, but
// it was too repetitive, so
// the Rust team programmed the compiler
// to infer the lifetime in some situations.
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}

struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn main() {
  let s1 = String::from("abcd");
  let s2 = "xyz";

  println!("the longest string is {}", longest(s1.as_str(), s2));

  let s1 = String::from("long string is long");

  {
    let s2 = String::from("xyz");

    // The lifetime 'a will be the lifetime of s2
    // because of all arguments that are passed to longest,
    // s2 is that that has smallest lifetime.
    // Even though longest will return a refence to s1
    // because it is the longer string,
    // result will only live until s2 goes out of scope
    // because the compiler can't know which
    // string reference longest would return beforehand.
    //
    // In another words:
    //
    // The lifetime of the reference returned by longest
    // will be the smallest lifetime of its arguments.
    let result = longest(s1.as_str(), s2.as_str());

    println!("the longest string is {}", result);
  } // result is not valid anymore

  // Another example
  let s1 = String::from("long string is long");

  let result;

  {
    let s2 = String::from("xyz");
    // We know that longest will return a reference
    // to s1 in this case but the compiler can't know that.
    // Because of that, result will live as long as s2,
    // which is the smallest lifetime.
    result = longest(s1.as_str(), s2.as_str());
  }
  // If we use result starting from here,
  // the code would not compile
  // dbg!(&result);

  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split(".").next().expect("could not find a '.'");

  // ImportantExcerpt can contain a &str
  // because it does not outlive it.
  let i = ImportantExcerpt {
    part: first_sentence,
  };
}
