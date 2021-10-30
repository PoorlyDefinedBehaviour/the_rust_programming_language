// Smart pointers
//
// A pointer is a general concept for a variable that contains
// and address in memory. This address refers to, or points at,
// some other data.
// The most common kind of pointer in Rust is a reference.
// References are indicated by the & symbol and borrow the
// value they point to. They don't have any special capabilities
// other than referring to data. Also, they don't have any
// overhead and are the kind of pointer we use most often.
//
// References summary:
//
// References use &.
// References don't do anything besides borrowing and pointing
// to a value.
// They don't have any overhead(probably is just a small copy of a pointer).
// --
//
// Smart pointers
//
// Smart pointers, on the other hand, are data structures that
// not only act like a pointer but also have additional
// metadata and capabilities.
// In Rust, the different smart pointers defined in the standard
// library provide functionality beyond that provided by references.
//
// One difference between references and smart pointers is that
// references are pointers that only borrow data,
// while smart pointers can also own data.
//
// Unexpected examples of smart pointers:
//
// String and Vec<T> are smart pointers because they own some
// memory and allow us to manipulate it. They also have metadata
// such as the capacity and extra capabilities or guarantees.
//
// Smart pointers are usually implemented using structs.
// The characteristic that distinguishes a smart pointer
// from an ordinary struct is that smart pointers implement
// the Deref and Drop traits.
//
// The Deref trait allows an instance of the smart pointer struct
// to behave like a reference so you can write code that works
// with either references or smart pointers.
//
// The Drop trait allows you to customize the code that is run
// when an instance of the smart pointers goes out of scope.
//
// NOTE: Drop kinda acts like a C++ destructor and is usually used for RAII.
//
// More examples of smart pointers defined by the standard library:
//
// Box<T> for allocating values on the heap.
// Rc<T>, a reference counting type that enables multiple ownership.
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that
// enforces the borrowing rules at untime instead of compile time.

fn main() {
  println!("Hello, world!");
}
