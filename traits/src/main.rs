#![allow(unused)]
// A trat tells the Rust compiler about functionality a
// particular type has and can share with other types.
// We can use traits to define shared behaviour in an abstract way.
// We can use trait bounds to specify that a generic type
// can be any type that has certain behaviour.

// A type's behaviour consists of the methods we can
// call on tha type.
// Different types share the same behaviour if we
// can call the same methods on all of those types.
use std::fmt::Display;

trait Summary {
  fn summarize_author(&self) -> String;

  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.summarize_author())
  }
}

struct NewsArticle {
  headline: String,
  location: String,
  author: String,
  content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
  }

  fn summarize_author(&self) -> String {
    self.author.clone()
  }
}

struct Tweet {
  username: String,
  content: String,
  is_reply: bool,
  is_retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }

  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

// impl Trait is syntax sugar for
// notify<T: Trait>(item &T)
fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize())
}

// item_1 and item_2 do not need to have the same type
fn notify_2(item_1: &impl Summary, item_2: &impl Summary) {}

// item_1 and item_2 DO NEED to have the same type
fn notify_3<T: Summary>(item_1: &T, item_2: &T) {}

// Multiple trait bounds
// item must implement Summary and Display
fn notify_4(item: &(impl Summary + Display)) {}

// same as above
fn notify_5<T: Summary + Display>(item: &T) {}

// Trait bounsd with where clauses
// Functions with multiple generic parameters
// can contain lots of trait bounds,
// making the function signature hard to read.
// For this reason, Rust has alternate syntax for specifying
// trait bounds inside a where clause
// after the function signature.
//
// Instead of writing:
fn some_function<T: Display + Clone, U: Clone + std::fmt::Debug>(t: &T, u: &U) {}

// We can write:
fn some_function_2<T, U>(t: &T, u: &U)
where
  T: Display + Clone,
  U: Clone + std::fmt::Debug,
{
}

// Returning types that implement traits
// We can also use the impl Trait syntax in the return position
// to return a value of some type that implements
// a trait.
// impl Summary in means that we will return some type
// that implements the Summary trait
//
// NOTE:
// We cannot return different types in a function
// that returns impl Trait.
// If we had a branch that returned a Tweet and other
// that returned a NewsArticle in the following function,
// the code would not compile.
//
// if something {
//  Tweet { ... }
// else {
//  NewsArticle { ... }
// }
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    is_reply: false,
    is_retweet: false,
  }
}

// Using trait bounds to conditionally implement methods
// By using a trait bound with an impl block that uses generic
// type parameters, we can implement methods only for types
// that implement the specified traits.
#[derive(Debug)]
struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

// cmp_display can called on Pair<T>
// as long as T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x >= self.y {
      println!("the largest member is x = {}", self.x);
    } else {
      println!("the largest member is y = {}", self.y);
    }
  }
}

// We can also implement a trait for any type that implements
// another trait.
// This is called a blank implementation.
// The standard library implements the ToString trait
// on any type that implements the display trait, for example.
trait MyTrait {
  fn f(&self) -> String;
}

// Any type that implements Display, automatically
// implements MyTrait
impl<T: Display> MyTrait for T {
  fn f(&self) -> String {
    format!("{}", self)
  }
}

fn main() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    is_reply: false,
    is_retweet: false,
  };

  notify(&tweet);
}
