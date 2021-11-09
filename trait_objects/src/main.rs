// Using trait objects that allow for values of different types
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

fn main() {
  println!("Hello, world!");
}
