// Treating smart pointers like regular references
// with the Deref trait
//
// Implementing the Deref trait allows us to customize the beaviour
// of the derefence operator(*).

fn main() {
  // A regular reference is a type of pointer, and one way to think
  // of a pointer is as an arrow to a value stored somewhere else.
  let x = 5;
  // Creating a pointer to x.
  let y = &x;
  assert_eq!(5, x);
  // Deferencing the pointer to get the value that is points to.
  assert_eq!(5, *y);

  // Using Box<T> like a reference
  let x = 5;
  // Storing 5 on the heap and pointing to it.
  // 5 is copied automatically because i32 implements the Copy trait,
  // if x was a String, we would have to call clone().
  let y = Box::new(x);

  assert_eq!(5, x);
  // We can dereference Box<T> because it implements
  // the Deref trait.
  assert_eq!(5, *y);

  struct MyBox<T>(T);

  impl<T> MyBox<T> {
    // We are not storing T on the heap for simplicity.
    fn new(x: T) -> Self {
      Self(x)
    }
  }

  // Since MyBox<T> implements Deref,
  // we can use the deference operator(*) on it.
  impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }

  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  // Since MyBox<T> implements Deref, when we write *MyBox<T>,
  // the compiler will actually call deref method
  // and apply the deference operator to the reference returned
  // by deref.
  // *(MyBox<T>::deref())
  assert_eq!(5, *y);

  // Implitit Deref coertions with functions and methods
  //
  // Deref coercion is a convenience that Rust performs
  // on arguments that implement the Deref trait
  // to functions and methods.
  //
  // Deref coercion converts such a type into a reference
  // to another type. For example, deref coercion can convert
  // &String to &str because String implements Deref trait such that
  // it returns &str.

  fn hello(name: &str) {
    println!("Hello, {}!", name);
  }

  // MyBox<T> implements Deref and String also implements Deref
  // so Rust will transform this call into:
  // (MyBox<T>::deref()).deref()
  hello(&MyBox::new(String::from("Rust")));

  // Rust can also coerce &mut to &.
}
