// Reference cycles can leak memory
//
// Rust's memory safety guarantees make it difficult,
// but not impossible, to accidentally create memory
// that is never cleaned up(known as memory leak).
//
// By using Rc<T> and RefCell<T>, it's possible to create
// references where items refer to each other in a cycle,
// causing memory to be leaked, because the reference count
// will never reach 0 so the values will never be dropped.
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List<T> {
  Cons(T, RefCell<Rc<List<T>>>),
  Nil,
}

impl<T> List<T> {
  fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
    match self {
      List::Cons(_, item) => Some(item),
      List::Nil => None,
    }
  }
}

fn main() {
  use List::*;

  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

  dbg!(Rc::strong_count(&a)); // 1
  dbg!(a.tail()); // Some(RefCell{ value: Nil })

  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

  dbg!(Rc::strong_count(&a)); // 2
  dbg!(Rc::strong_count(&b)); // 1
  dbg!(b.tail()); // Some(RefCell {value: Cons(5, RefCell {value: Nil})})

  // Now a points to to b and b points to a
  *a.tail().unwrap().borrow_mut() = Rc::clone(&b);

  dbg!(Rc::strong_count(&b)); // 2
  dbg!(Rc::strong_count(&a)); // 2

  // This overflows the stack because the dbg! macro
  // will follow the cycle.
  // dbg!(a.tail());
}
