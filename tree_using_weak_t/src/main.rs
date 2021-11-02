// Creating a tree data structure: a node with child nodes
use std::{
  cell::RefCell,
  rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node<T> {
  // We hold a Weak pointer to the parent
  // because the parent owns its children, but each child
  // does not own its parent.
  // This means, if a parent goes out of scope, its children
  // should be dropped but not the other way around.
  parent: RefCell<Weak<Node<T>>>,
  value: T,
  children: RefCell<Vec<Rc<Node<T>>>>,
}

fn main() {
  let leaf = Rc::new(Node {
    parent: RefCell::new(Weak::new()),
    value: 3,
    children: RefCell::new(vec![]),
  });

  let branch = Rc::new(Node {
    parent: RefCell::new(Weak::new()),
    value: 5,
    // leaf has two owns because we are cloning it:
    // leaf itself and branch.
    children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  // Rc::downgrade(&Rc<T>) creates a Weak<T> reference
  // to the Rc<T>.
  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

  dbg!(&leaf);
  // dbg! does not overflow the stack, because even though we have
  // a cycle, it is a weak reference.
  //
  //   Node {
  //     parent: RefCell {
  //         value: (Weak),
  //     },
  //     value: 3,
  //     children: RefCell {
  //         value: [],
  //     },
  //   }
}
