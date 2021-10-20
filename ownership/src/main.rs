// All programs have to manage the way they use a computer's
// memory while running.
// Some languages have garbage collection that constantly looks
// for no longer used memory as the program runs;
// in other languages, the programmer must explictly allocate
// and free the memory. Rust uses a third approach: memory is
// managed through a system of ownership with
// a set of rules that the compiler checks at compile time.
// None of the ownership features slow down your program
// while it's running.
//
// The stack and the heap
//
// Both the stack and the heap are parts of memory
// that are available to your code to use at
// runtime, but they are structured in different ways.
// The stack stores values in the order it gets them
// and removes the values in opposite order.
// This is referred to as last in, first out.
//
// All data stored on the stack must have a known, fixed size.
// Data with an unknown size at compile time or a size that
// might change must be stored on the heap instead.
//
// The heap is less organized: when you put data on the heap,
// you request a certain amount of space.
// The memory allocator finds an empty spot in the heap that is
// big enough, marks it as being in use and returns a pointer,
// to that memory address.
// This process is called allocating on the heap and is sometimes
// abbreviated as just allocating. 
// Pushing values on the stack is not considered allocating.
// Because the pointer is a known, fixed size, you can store
// the pointer on the stack, but when you want the actual data,
// you must follow the pointer.
//
// Pushing to the stack is faster than allocating on the heap
// because the allocatot never has to search for a place to
// store new data.
//
// Accessing data on the stack is also faster because you don't
// have to follow a pointer to get there.
//
// ---
// Ownership rules
// 
// Each value in Rust has a variable that's called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
fn main() {
  // Variable scope
  //
  // A scope if the stange within a program for which an item
  // is valid.
  { // starts new scope
    // [s] is valid until the end of the current scope
    let s = "hello";
    dbg!(&s);
  } // ends current scope

  // The String type
  //
  // This type is allocated on the heap and as such is able
  // to store an amount of text that is unknown at compile time.
  // We can create a String from a string literal using the from function:dbg!
  let mut s = String::from("hello");

  dbg!(&s); // hello

  // String can be mutated unlike string literals
  s.push_str(", world!"); 

  dbg!(&s); // hello, world!

  // Memory and allocation
  //
  // In the case of a string literal, we know the contents at compile tine,
  // so the text is hardcoded directly into the final executable.
  // This is why string literals are fast and efficient.
  // But these properties only come from the string literal's immutability.
  // We can not do that for String because we don't know it's size and contents at compile time.
  //
  // With the String type, in order to support a mutable, glowable piece of text,
  // we need to allocate an amount of memory on the heap, unknown at compile time
  // to hold the contents.
  //
  // Unlike languages that use garbage collectors or manual memory management,
  // Rust frees memory once the variable that owns it goes out of scope.
  {
    let s = String::from("hello"); 
    dbg!(&s);
  } // [s] goes out of scope and its memory is freed
  // When a variable goes out of scope, Rust calls a special function
  // called drop, and it's where the author of the String type can put
  // the code to return the memory.
  //
  // NOTE: in C++, this pattern of deallocating resources at the end
  // of an item's lifetime is called Resource Acquisition is Initialization(RAII).

  // Move
  // The value from x is copied because integers are simple values
  // with a known, fixed-size, and these two 5 values
  // are pushed onto the stack.
  let x = 5;
  let y = x; // copies 5 
  dbg!(&y); // 5

  let s1 = String::from("hello");
  let s2 = s1;
  // dbg!(&s1); --> compile error: borrow of moved value
  dbg!(&s2); // hello
  
  // A string is made of three parts:
  // pointer to the data in the heap
  // length of the string
  // capacity of the tring
  //
  // When we assign s1 to s2, the String data is copied,
  // meaning we copy the pointer, the length and the capacity
  // that are on the stack. We do not copy the data on the heap
  // that the pointer refers to.
  //
  // Since now we have two variables that own the same data,
  // Rust considers s1 invalid after the assignment to s2
  // to avoid a double free that would happen when both
  // variables went out of scope.
  //
  // This is called a moved, a shallow copy is made
  // and the previous owner of the data is no longer considered valid,
  // like its contents had been moved somewhere else.
  //
  // NOTE: Rust will never automatically create deep copies of your data.
  // Therefore, any automatic copying can be assumed to be inexpensive
  // in terms of runtime performance.

  // If we do want to deeply copy the heap data of the String,
  // not just the stack data, we can use a common method
  // called clone.
  let s1 = String::from("hello");
  let s2 = s1.clone();
  dbg!(&s1); // hello
  dbg!(&s2); // hello

  // Stack-Only data: Copy
  // Why is the value not moved?
  //
  // The reason is that types such as integers that have a known size
  // at compile time are stored entirely on the stack, so copies of
  // the actual values are quick to make.
  // That means there's no reason we would want to prevent x from
  // being valid after we create the varible y.
  // In other words, there's no difference between deep and shallow copying
  // here, so calling clone wouldn't do anything different.
  //
  // Rust has a special annotation called the Copy trait that we
  // can place on types that are stored on the stack.
  // If a type implements the Copy trait, the previous owner
  // of the value if still usable after assignment.
  //
  // Rust won't let us annotated a type with the Copy trait if the type,
  // or any of its parts, has implemented the Drop trait.
  let x = 5;
  let y = x;
  dbg!(x); // 5
  dbg!(y); // 5
}
