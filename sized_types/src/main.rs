#[allow(unused)]
// Dynamically sized types and the Sized trait
//
// Due to Rust's need to know certain details, such as how much space
// to allocate for a value of a particular type, there is a corner
// of its type system that can be confusing:
// the concept of dynamically sized types. Sometimes referred to as
// DSTS or unsized types, these typesl et us write code using values
// whose size we can only know at runtime.
//
// An example of a dynamically sized type is str.
// We can't know how much long the string is until runtime,
// meaning we can't create a variable of type str, nor can we
// take an argument of type str.
//
// The following code does not compile:
//
// let s1: str = "Hello there!";
// let s2: str = "How's it going?";
//
// Rust needs to know how much memory to allocate for any value of
// a particular type, and all values of a type must use the same
// amount of memory. If rust allowed us to write this code,
// these two str values would need to take up the same amount of space.
// But they have different lenghts: s1 needs 12 bytes of storage and s2
// needs 15. This is why it's not possible to create a variable
// holding a dynamically sized type.
//
// To work around this, we can use &str instead os str.
// &str is two values: the address of the str and its length.
// As such, we can know the size of a &str value at compile time,
// it's twice the length of a usize(pointer + length).
//
// The Sized trait
//
// To work with dynamically sized types, Rust has a particular trait
// called the Sized trait to determine whether or not a type's size
// is known at compile time. This trait is automatically implemented
// for everything whose sizd is known at compile time.
//
// In addition, Rust implicitly adds a bound on Sized to every generic
// function.
//
// That is, a generic function like this:
fn generic<T>(t: T) {
  unimplemented!();
}

// Is actually treated as though we had written this:
fn generic_1<T: Sized>(t: T) {
  unimplemented!();
}

// By default, generic functions will work only on types that
// have a known size at compile time.
//
// However, we can use the following special syntax to relax this restriction:
//
// T: ?Sized means that T may or may not be sized.
//
// Since T may not be sized, we need to put it behind a pointer type
// so we use t has type &T instead of T.
fn generic_2<T: ?Sized>(t: &T) {
  unimplemented!();
}
fn main() {
  println!("Hello, world!");
}
