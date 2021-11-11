#![allow(unused)]
// Using trait objects that allow for values of different types
//
// Object safey is required for trait objects
//
// You can only make object-safe trait objects.
// A trait is object safe if all the methods defined in the trait
// have the following properties:
//
// The return type isn't Self.
// Because trait objects forget about the concrete type implementing
// it and it a method had the concrete type as its return type,
// the trait would have to know which type is implementing it.
// NOTE: don't we know the concrete type that is implementing it
// because of how the vtable is constructed?
//
// There are no generic type parameters.
// Same reason as the return type of a method can't be Self,
// the trait would have to remember what concrete type it is
// working with.
//
// The standard library Clone trait is an example of
// an trait thats is not object safe.
//
// pub trait Clone {
//  -- not object safe because it returns Self in one
//  -- of its methods.
//  fn clone(&self) -> Self;
// }
//
// We will create and example graphical user interface(GUI) tool
// that iterates through a list of items, calling a draw method
// on each one to tdraw it to the screen(a common technique for GUI tools).

// To implement the behaviour we want gui to have, we'll define a trait
// named Draw that will have one method named draw.
//
// A trait object points to both an instance of a type implementing
// our specified trait as well as a table(vtable) used to look up trait methods
// on that type at runtime(dynamic dispatch). We create a trait object
// by specifying some sort of pointer, such as a & reference or a Box<T>
// smart pointer, then the dyn keyword, and then specifying the relevant trait.
trait Draw {
  fn draw(&self);
}

struct Screen {
  // [components] is a list of any type inside a Box<T>
  // that implments Draw.
  //
  // We need Box<T> here because the size of a Box<T>
  // is always known, while the syze of dyn Draw is not known.
  components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

struct Button {
  width: u32,
  height: u32,
  label: String,
}

impl Draw for Button {
  fn draw(&self) {
    // draw button on the screen
  }
}

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // ...
  }
}

fn main() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };

  screen.run();
}
