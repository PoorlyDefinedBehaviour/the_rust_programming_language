// Using message passing to transfer data between threads
//
// One increasingly popular approach to ensuring safe concurrency
// is message passing, where threads or actors communicate by sending
// each other messages containing data(Go also does that).

// ┌──────────────────────┐                                            ┌──────────────────────┐
// │                      │                                            │                      │
// │                      │                                            │                      │
// │                      │                                            │                      │
// │                      │                                            │                      │
// │                      │            ┌────────────────────┐          │                      │
// │                      │            │                    │          │                      │
// │       Thread A       │            │                    │          │        Thread B      │
// │                      ├───────────►│                    ├──────────►                      │
// │                      │            │     Message        │          │                      │
// │                      │            │                    │          │                      │
// │                      │            │                    │          │                      │
// │                      │            └────────────────────┘          │                      │
// │                      │                                            │                      │
// │                      │                                            │                      │
// │                      │                                            │                      │
// │                      │                                            │                      │
// └──────────────────────┘                                            └──────────────────────┘
//
// One major tool Rust has for accomplishing message-sending concurrency
// is the channel.
//
// A channel has two halves: a transmitter and a receiver.
// The transmitter is used to send data and
// the receiver is used to receive data send from the
// transmitter.
//
// A channel is said to be closed if either the transmitter
// or receiver half is dropped.
use std::{sync::mpsc, thread};

// Running in main thread
fn main() {
  // mpsc = Multiple Producer, Single Consumer
  let (tx, rx) = mpsc::channel();

  // Spawning thread A
  thread::spawn(move || {
    let value = String::from("hi");
    // Sender::send returns Result<T,E> because the receiving
    // end may already have been dropped in which case,
    // an error will be returned.
    tx.send(value).unwrap();
  });

  // Receiver::recv blocks the ccurrent thread execution
  // until a value is sent down the channel.
  // Once a alue is sent, Receiver::recv will return it in a
  // Result<T, E>. When the sending end of the channel
  // closes, Receiver::recv will return an error to signal
  // that no more values will be coming.
  //
  // Receiver also has the try_recv method which doesn't block
  // the current thread execution but will return a Result<T, E>
  // immediatelly: an Ok value holding a message if one is available
  // and an Err value if there aren't any messages this time.
  let received = rx.recv().unwrap();
  //          Receiver::recv          Sender::send
  //                │                       │
  //                │                       │
  // ┌───────────┐  │                       │ ┌────────────┐
  // │           │  │                       │ │            │
  // │           │  │ ┌─────────────────┐   │ │            │
  // │   main    │  │ │                 │   │ │            │
  // │   Thread  ◄──┴─┤       hi        ◄───┴─┤   Thread A │
  // │           │    │                 │     │            │
  // │           │    └─────────────────┘     │            │
  // │           │                            │            │
  // │           │                            │            │
  // └───────────┘                            └────────────┘

  println!("got: {}", received);
}
