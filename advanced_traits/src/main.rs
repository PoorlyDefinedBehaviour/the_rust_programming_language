// Specifying placeholder types in trait definitions with associated types
//
// Associated types connect a type placeholder with a trait such that
// the trait method definitions can use these placeholder types
// in their signatures.
//
// The implementor of a trait will specify the concrete type to be used
// in this type's place for the particular implementation.
//
// NOTE: What's the difference between an associated type and a generic type,
// and why traits don't use generic type parameters like trait<T> MyTrait for example?
//
// Why trait use associated types instead of generic types
//
// If we used generic types in a trait:
//
// trait Iterator<T> {
//  fn next(&mut self) -> Option<T>;
// }
//
// we would be able to have multiple implementations of the trait:
//
// impl Iterator<String> for Counter { ... }
// impl Iterator<u32> for Counter { ... }
//
// which would force us to especify the type parameter when calling
// Iterator<T>::next because T could be more than one type.
//
// Associated types solve this problem since we will only be able
// to implement the trait once for each type.
//
// impl Iterator for Counter {
//  type Item = u32;
//
//  fn next(&mut Self) -> Option<Self::Item> { ... }
// }
trait MyIterator {
  // [Item] is a placeholder type which will be defined
  // by the implementer of this trait.
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}
fn main() {
  println!("Hello, world!");
}
