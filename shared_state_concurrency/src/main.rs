// Shared-state concurrency
//
// In a way, channels are similar to single ownership,
// because once you transfer a value down a channel,
// you should no longer use that value.
// Shared memory concurrency is like multiple ownership:
// multiple threads can access the same memory location
// at the same time.
//
// Using mutexes to allow access to data from one thread at a time
//
// Mutex is an abbreviation for mutual exclusion, as in,
// a mutex allows only one thread to access some data at
// any given time. To access the data in a mutex, a thread must first
// signal that it wants acess by asking to acquire the mutex's lock(Mutex::lock).
// The lock is a data structure that is part of the mutex that keeps track
// of who currently has exclusive access to the data.
// Therefore, the mutex is described as guarding the data it holds via
// the locking system.
//
// Mutex rules:
//
// You must attempt to acquire the lock before using the data.
// When you're done with the data that the mutex guards,
// you must unlock the data so other threads can acquire the lock.
use std::{
  sync::{Arc, Mutex},
  thread,
};

fn main() {
  // The value being guarded by the mutex is 5.
  let mutex = Mutex::new(5);

  {
    // Mutex::lock blocks the current thread until the lock
    // can be acquired.
    //
    // Mutex::lock returns a smart pointer called MutexGuard
    // wrapped in a LockResult. MutexGuard implements Deref
    // so we can use the dereference operator(*) to reach
    // the value being held by it. The MutexGuard smart pointer
    // also has a Drop implementation that releases the lock
    // automatically when MutexGuard goes out of scope.
    let mut num = mutex.lock().unwrap();
    *num = 6;
  }

  // The value being guarded by the mutex is 6.
  dbg!(&mutex);

  // Arc<T> is like Rc<T> but thread-safe.
  let counter = Arc::new(Mutex::new(0));

  // We will increment counter 10 times using 10 different threads.
  let handles = (0..10)
    .map(|_| {
      // Cloning the atomic reference to [counter]
      // because we will move it to another thread.
      let counter = Arc::clone(&counter);

      // The `move` keyword moves [counter] to the thread we
      // are spawning, and it is ok because [counter]
      // is atomically reference counted.
      thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
      })
    })
    .collect::<Vec<_>>();

  for handle in handles {
    handle.join().unwrap();
  }

  dbg!(*counter.lock().unwrap()); // 10
}
