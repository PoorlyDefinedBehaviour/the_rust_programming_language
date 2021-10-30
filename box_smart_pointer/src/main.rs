// Using Box<T> to point to data on the heap
//
// The most straightforward smart pointer is a box,
// whose type is written Box<T>.
// Boxes allow you to store data on the heap rather than the stack.
// What remains on the stack is the pointer to the heap data.
//
// Stack                  Heap
// Box<T> {
//   value: *T ---------> T
// }
//
// Boxes don't have performance overhead, other than storing
// their data on the heap instead of the stack.
//
// Examples of when to use Box<T>:
//
// When you have a type whose size can't be known at compile time
// and you want to use a value of that type in a context
// that requires an exact size.
//
// NOTE: Recursive types are an example of that
//
// enum Expr {
//   Int(i32)
//   Add(Box<Expr>, Box<Expr>)
// }
//
// When you have a large amount of data and you want to transfer
// ownership but ensure the data won't be copied when you do so.
//
// When you want to own a value and you care only that
// it's a type that implements a particular trait rather
// than being of a specific type.
//
// NOTE: Probably talking about Box<dyn Trait>

// A const list is a data structure that comes the Lisp programming
// language and its dialects.
// In Lisp, the cons function(short for construct function)
// constructs a new pair from its two arguments, which usually
// are a single value and another pair.
// These pairs containing pairs form a list.
//
// We could defined list like this, but it wouldn't compile
// because List doesn't have a known size.
//
// To determine how much space to allocate, Rust goes through
// each of the variants too se which variant needs the most spaces.
// The size of the type is the biggest size needed by one of the variants.
//
// But since we are declaring a recursive type,
// Rust would enter an infinite loop while trying to calculate
// the size for the type.
//
// Rust can't know the size of List, because it is recursive
// and we could have and infinite list.
//
// Cons(1, Cons(2, Cons(3, Cons(...))))
//
// enum List<T> {
//   Cons(T, List<T>),
//   Nil,
// }
//
// We can fix it by boxing the recursive type,
// since Rust always knows the size of a Box<T> and
// it's size does not change based on the type of the
// value it is pointing to.
#[derive(Debug)]
enum List<T> {
  // The size of List<T>
  // if the size necessary to
  // store T + a pointer to a List<T>
  Cons(T, Box<List<T>>),
  Nil,
}

fn main() {
  // 5 is store on the heap and b is a struct
  // stored on the stack that has a pointer to 5.
  let b = Box::new(5);
  println!("b = {}", b);

  use List::*;

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

  dbg!(&list);
}
