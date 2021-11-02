// Preventing reference cycles: Turning an Rc<T> into a Weak<T>
//
// We saw that calling Rc::clone(&Rc<T>) increases the strong_count
// of an Rc<T> instance, and an Rc<T> instance of inly cleanup up
// if its strong_count is 0.
//
// We can also create weak references to the value within an Rc<T> instance
// by calling Rc::downgrade(&Rc<T>).
//
// When we call Rc::downgrade, we get a smart pointer of type
// Weak<T>. Instead of increasing the strong_count in the Rc<T>
// instance by 1, calling Rc::downgrade increases the weak count by 1.
//
// The Rc<T> type uses weak_count to keep count of how many
// Weak<T> references exist, similar to strong_count. The difference
// is that weak_count doesn't need to be 0 for the Rc<T> instance
// to be cleanup up.
//
// NOTE: This looks like C++'s std::weak_ptr
//
// Strong references(Rc::clone) express shared ownership of an
// Rc<T> instance.
//
// Weak references(Rc::downgrade) don't express ownership of an
// Rc<T> instance.
//
// The value that Weak<T> references might have been dropped,
// so to use the value, we need to call the Weak<T>::upgrade,
// whcih will return an Option<Rc<T>>.
// We get Some when the value has not been dropped and None and it
// has been dropped.
//

fn main() {
  println!("Hello, world!");
}
