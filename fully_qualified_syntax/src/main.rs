// Rust allows us to implement traits that have methods
// with the same name and to implement methods
// that have the same name as methods that are defined
// in traits.
//
// When calling methods with the sanem name, we need to tell
// Rust which method to use.

trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) {
    println!("this is your captain speaking");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("up");
  }
}

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

fn main() {
  let person = Human;

  // When we call fly on instance of Human, the compiler defaults
  // to calling the method that is directly implemented on the type.
  person.fly(); // *waving arms furiously*

  // To call the fly methods from the other traits, we need to use
  // more explicit syntax:
  Pilot::fly(&person); // this is your captain speaking
  Wizard::fly(&person); // up

  // We could call the method that's implemented in Human
  // using fully qualified syntax as well:
  Human::fly(&person); // *waving arms furiously*
                       // Note that we pass person as argumentto these methods
                       // because they all take &self as parameter.

  dbg!(Dog::baby_name()); // Spot
                          // If we want to call the Animal implementation for Dog
                          // we need to cast to the Animal trait because
                          // the trait method does not take &self as parameter.
  dbg!(<Dog as Animal>::baby_name()); // puppy
}
