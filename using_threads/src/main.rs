use std::thread;
use std::time::Duration;

fn main() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      // TODO: is println! thread safe?
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    // We added thread::sleep to give the other thread
    // a chance to run.
    thread::sleep(Duration::from_millis(1));
  }

  // Wait to the thread to be done.
  //
  // Calling JoinHandle::join block the thread currently running
  // until the thread representeed by the handle terminates.
  handle.join().unwrap();
}
