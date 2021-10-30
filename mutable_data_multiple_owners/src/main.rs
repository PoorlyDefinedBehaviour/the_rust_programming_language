// Having multiple owners of mutable data by combining Rc<T>
// and RefCell<T>
//
// Rc<T> lets you have multiple owners of some immutable data.
// If you have an Rc<T> that holds a RefCell<T>,
// you can have multiple owners to something the can be mutated.
use std::{cell::RefCell, rc::Rc};

// We are wrapping T in a RefCell to make it mutable.
#[derive(Debug)]
enum List<T> {
  Cons(Rc<RefCell<T>>, Rc<List<T>>),
  Nil,
}

fn main() {
  use List::*;

  let value = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

  let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

  let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

  println!("a before = {:?}", a);
  println!("b before = {:?}", b);
  println!("c before = {:?}", c);

  *value.borrow_mut() += 10;

  println!("a after = {:?}", a);
  println!("b after = {:?}", b);
  println!("c after = {:?}", c);
}
