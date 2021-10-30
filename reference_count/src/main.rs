// Rc<T>, the reference counted smart pointer
//
// Useful when a value has multiple owners.
//
// To enable multiple ownership, Rust has a type called Rc<T>,
// which is an abbreviatin for reference counting.
// The Rc<T> type keeps track of the number of references
// to a value to determine whether or not the value
// is still in use. If there are zero references to a value,
// the value can be cleaned up without any references becoming invalid.
// NOTE: kinda like C++ std::shared_ptr
//
// Nota that Rc<T> is only for use in single-threaded scenarios.
use std::rc::Rc;

// Gotta be careful to not create circular references
// because that will stop the data pointed by the Rc
// from being cleaned up.
enum List<T> {
  Cons(T, Rc<List<T>>),
  Nil,
}

fn main() {
  use List::*;

  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  // a is only referenced by itself.
  dbg!(Rc::strong_count(&a)); // 1

  // If we need a reference to a Rc, we can use Rc::clone.
  // Rc::clone increases the reference count
  // and does not make a deep copy of the data.
  let b = Cons(3, Rc::clone(&a));

  // a is referenced by itself an b.
  dbg!(Rc::strong_count(&a)); // 2
  {
    let c = Cons(4, Rc::clone(&a));

    // a is referenced by itself, b and c.
    dbg!(Rc::strong_count(&a)); // 3
  } // c goes out of scope and drop is called
    // to decrease the reference count.

  // a is referenced by itself and b.
  dbg!(Rc::strong_count(&a)); // 2
}
