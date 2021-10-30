#![allow(unused)]
// Running code on cleanup with the Drop trait
//
// The Drop trait lets you customize what happens when a value
// is about to go out of scope. The Drop implementation can be
// used to release resources like files or network connections.
//
// For example, when a Box<T> is dropped it will deallocate the
// space on the heap that the box points to.
struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data {}", self.data);
  }
}

fn main() {
  {
    let c = CustomSmartPointer {
      data: String::from("hello world"),
    };
    let d = CustomSmartPointer {
      data: String::from("hello world 2"),
    };
  } // d.drop() and c.drop() is called here
    // variables are dropped in reverse order oof thei creation
    // so d is dropped before c

  // Rust does not allow us to call drop manually, but we
  // can use std::mem::drop if we want to drop a value manually
  // because Rust would still call drop after the value
  // goes out of scope possibly causing a double free.
  //
  // c.drop(); -- compile error
  //
  let c = CustomSmartPointer {
    data: String::from("some data"),
  };

  println!("CustomSmartPointer created.");
  drop(c);
  println!("CustomSmartPointer dropped before the end of main.");
}
