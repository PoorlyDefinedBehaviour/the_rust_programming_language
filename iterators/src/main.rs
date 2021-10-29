// The iterator pattern allows you to perform some task on a sequence
// of items in turn. An iterator is responsible for the logic
// of iterating over each item and determining when the
// sequence has finished.
//
// The iterator pattern allows us to write code that works
// on many different collections as long as they
// implement the Iterator trait.

// All iterators implement a trait Named Iterator that is defined by
// the standard library.
// It kinda looks like this:
pub trait Iterator {
  // Item is an associated type
  type Item;

  fn next(&mut self) -> Option<Self::Item>;

  // more methods with default implementations...
}

fn main() {
  // In Rust, iterators are lazy, meaning they have no effect until
  // you call methods that consume the iterator.
  let v1 = vec![1, 2, 3];
  // Calling .iter() does nothing because iterators are lazy.
  let v1_iter = v1.iter();

  // Since v1_iter is being used, the computation will
  // take place now.
  for value in v1_iter {
    println!("{}", value);
  }

  let v1 = vec![1, 2, 3];

  // v1_iter needs to be mutable because Iterator::next
  // takes a mutable reference and we are going to call it.
  let mut v1_iter = v1.iter();

  // When iterator is not done iterating through the collection,
  // Some(Iterator::Item) is returned.
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  // When iterator is done iterating through the collection,
  // None is returned.
  assert_eq!(v1_iter.next(), None);

  // Note that v1_iter.next() returned Option<&Iterator::Item>,
  // if we wanted to take ownership of the values returned by
  // Iterator::next, we could call v1.into_iter() instead.
  // Similarly, if we wanted to iterate over mutable references,
  // we could call v1.iter_mut().

  // Methods that consume the iterator
  //
  // The Iterator trait has a number of different methods
  // with default implementations provided by the standard library.
  // Some of these methods call the next method in their definition.
  //
  // Methods that call next called consuming adaptors, because
  // calling them uses up the iterator.
  // Iterator::sum for example, takes ownership of the iterator
  // and iterates through the items by calling next.
  // As it iterates through, it adds each item to a total and returns
  // the total when Iterator::next returns None because the iteration is
  // complete.
  let v1 = vec![1, 2, 3];
  assert_eq!(v1.iter().sum::<i32>(), 6);
}
