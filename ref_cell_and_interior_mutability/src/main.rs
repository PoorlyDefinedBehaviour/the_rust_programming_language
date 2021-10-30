// RefCell<T> and interior mutability pattern
//
// Interior mutability is a design pattern in Rust that allows
// you to mutate data even when there are immutable references
// to that data.
//
// To mutate data, the pattern uses unsafe code inside a
// data structure to bend Rust's usual rules that govern
// mutation and borrowing.
//
// We can use types that use the interior mutability pattern
// when we can ensure that the borrowing rules will be followed
// at runtime, even though the compiler can't guarantee that.
// The unsafe code involved is then wrapped in safe API,
// and the outer type is still immutable.
//
//
// Enforcing borrowing rules at runtime with RefCell<T>
//
// Unlike Rc<T>, the RefCell<T> type represents single ownership
// over the data it holds.
//
// Recall the borrowing rules:
//
// At any given time, you can have either(but not both of)
// one mutable reference or any number of immutable references.
// References must always be valid.
//
// With references and Box<T>, the borrowing rules invariants
// are enforced at compile time.
//
// With RefCell<T>, these invariants are inforced at runtime.
//
// You can use RefCell<T> when you know your code follows
// the borrowing rules but the compiler is not able to
// understand that.
//
// Recap of Box<T>, Rc<T> and RefCell<T>
//
// Box<T> and Rc<T> are checked at compile time.
// RefCell<T> is checked at runtime.
//
// Because RefCell<T> allows mutable borrows checked at runtime,
// you can mutate the value inside RefCell<T> and when the
// RefCell<T> is immutable.
//
// Mutating a value inside an immutable value is what's called
// the interior mutability pattern.
pub trait Messenger {
  fn send(&self, message: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &'a T, max: usize) -> Self {
    Self {
      messenger,
      max,
      value: 0,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: you are over your quota!");
    } else if percentage_of_max >= 0.9 {
      self
        .messenger
        .send("Urgent warning: you've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
      self
        .messenger
        .send("Warning: you've used up over 75% of your quota!");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  #[derive(Default)]
  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::default();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}

fn main() {
  println!("Hello, world!");
}
