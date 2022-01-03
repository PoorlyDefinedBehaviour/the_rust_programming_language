use std::{
  sync::{mpsc, Arc, Mutex},
  thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
  /// Holds a job that the thread should run.
  NewJob(Job),
  /// Tells the thread to exit its loop and stop.
  Terminate,
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// `size` is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is 0
  /// or greater than 1000.
  pub fn new(size: usize) -> Self {
    // NOTE: new does not return Result<T, E> for simplicity.
    assert!(size > 0 && size <= 1000);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    Self { workers, sender }
  }

  pub fn execute(&self, f: impl FnOnce() + Send + 'static) {
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Terminating workers");

    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("Stopping workers");

      if let Some(thread) = worker.thread.take() {
        println!("Stopping worker {}", worker.id);
        thread.join().unwrap();
      }
    }
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
    let thread = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv().unwrap();

      match message {
        Message::NewJob(job) => {
          println!("Worker {} got a job; executing.", id);

          job();
        }
        Message::Terminate => {
          println!("Worker {} was told to terminate", id);
          // Exit infinite loop.
          break;
        }
      }
    });

    Self {
      id,
      thread: Some(thread),
    }
  }
}
