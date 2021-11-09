// The Rust language has very few concurrency features.
// Almost every concurrency feature is part of the standard library,
// not the language. You can build your own concurrency features
// or use features built by others.
//
// However, two concurrency concepts are embedded in the language:
// the std::marker traits Sync and Send.
//
// Allowing transference of ownership between threads with Send
//
// The Send marker trait indicates that ownership of values of the
// the type implementing Send can be transferred between threads.
//
// Almost every Rust type is Send, but there are some exceptions,
// including Rc<T>, which cannot be Send because if you cloned
// an Rc<T> value and tried to transfer ownership of the clone
// to another thread, both threads might update the referece count
// at the same time. For this reason Rc<T> is implemented for
// use in single-threaded situations where you don't want to pay the
// thread-safe performance penalty.
//                     ┌─────────────┐
//                     │             │
//                     │             │
//                     │    Rc<T>    │
//             ┌───────►  count: ?   ◄────────────┐
//             │       │             │            │
//             │       │             │            │
//             │       │             │            │
//             │       │             │            │
//             │       └─────────────┘            │
//             │                                  │
// ┌───────────┴┐                           ┌─────┴──────┐
// │            │                           │            │
// │            │                           │            │
// │            │                           │            │
// │  Thread A  │                           │  Thread B  │
// │Rc::clone(.)│                           │Rc::clone(.)│
// │            │                           │            │
// │            │                           │            │
// │            │                           │            │
// └────────────┘                           └────────────┘
//
// Therefore, Rust's type system and trait bounds ensure that you can
// never accidentally send an Rc<T> value accross threads unsafely.
//
// Any type composed entirely of Send types is automatically marked
// as Send as well.
//
// Allowing access from multiple threads with Sync
//
// The Sync marker trait indicates that it is safe for the type
// implementing Sync to be referenced fro multiple threads.
//
// In other words, any type T is sync if &T is Send, meaning
// the immutable reference can be sent safely to another thread.
// Similar to Send, primitive types are Sync, and types composed
// entirely of types that are Sync are also Sync.

fn main() {
  println!("Hello, world!");
}
