// Closures: Anonymous functions that can capture their environment
//
// Rust's closures are anonymous functions you can save in a variable
// or pass as arguments to other functions. You can create the closure
// in one place and then call the closure to evaluate it in
// a different context. Unlike functions, closures can capture
// values from the scope in which they're defined.
//
// Each closure instance has its own unique anonymous type,
// even if two closures have the same signature, they don't
// have the same type.
//
// These two closures have different types even though
// we are calling both with an argument that has the i32 type:
//
// let f = |x\ x
//
// f(1)
//
// let g = |x| x
//
// f(1)
use std::thread;
use std::time::Duration;

// Cacher is used to memoize a computation
struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Self {
    Self {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn generate_workout(intensity: u32, random_number: u32) {
  let mut simulated_expensive_calculation = Cacher::new(|intensity| {
    println!("calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
  });

  if intensity < 25 {
    println!(
      "Today, do {} pushups!",
      simulated_expensive_calculation.value(intensity)
    );
    println!(
      "Next, do {} situps!",
      simulated_expensive_calculation.value(intensity)
    );
  } else {
    if random_number == 3 {
      println!("Take a break today! Remember to stay hydrated!");
    } else {
      println!(
        "Today, run for {} minutes!",
        simulated_expensive_calculation.value(intensity)
      )
    }
  }
}

fn main() {
  // Closure definitions will have one concrete type inferred for each
  // of their parameters and for thei return values.
  //
  // For example, if we call example_closure with a String
  // and then with another type, we will get an error.
  let example_closure = |x| x;

  // We are calling example_closure for the first time and
  // the argument type is String, so example_closure type
  // is inferred to String -> String
  let _ = example_closure(String::from("hello"));

  // let _ = example_closure(5); -- compile error

  // ---
  // Capturing the environment with closures
  //
  // Closures can capture their environment and
  // access variables from the scope in which they're defined.
  //
  // In Rust 2018, closures didn't have disjoint capture,
  // but in Rust 2021 they do.
  // https://doc.rust-lang.org/edition-guide/rust-2021/disjoint-capture-in-closures.html
  //
  // NOTE: we cannot capture the environment with functions
  let x = 4;

  let equal_to_x = |z| z == x;

  let y = 4;

  assert!(equal_to_x(y));
  // Closures can capture values from their environment in three ways,
  // which directly map to the three ways a function can take a parameter:
  // taking ownership, borrowing mutably, and borrowing immutably.
  // These are encoded in the Fn traits:
  //
  // FnOnce
  // Consumes the variables it captures from its enclosing scope,
  // known as the closure's environment. To consume the captured variables,
  // the closure must take ownership of these variables and move them
  // into the closure when it is defined.
  // The once part of the name represents the fact that the closure can't
  // take ownership of the same variables more than once, so it can
  // be called only once.
  //
  // FnMut
  // Can change the environment because it mutably borrows values.
  //
  // Fn
  // Borrows values from environment immutably.
  //
  // When a closure is created, Rust infers which trait to use
  // based on how the closure uses the values from the environment.
  //
  // All closures implement FnOnce because they can all be called
  // at least once.
  //
  // Closures that don't move the cpatured variables also implement FnMut.
  //
  // And closures that don't need mutable access to the captured variables
  // also implement Fn.
  //
  // We can force the closure to take ownership of the values
  // it uses from the environment by adding the move keyword
  // before the parameter list
  //
  // NOTE: move closures may still implement Fn or FnMut, even though
  // they capture variables by move. This is because the traits
  // implemented by a closure type are determined by what the
  // closure does with captures values, not how it captures them.
  let x = vec![1, 2, 3];

  let equal_to_x = move |z| z == x;

  // x cannot be used here because it has been moved into equal_to_x
  // because we added the move keyword to equal_to_x declaration.
  // println!("{:?}", x); -- compile error

  let y = vec![1, 2, 3];

  assert!(equal_to_x(y));

  // ---
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout(simulated_user_specified_value, simulated_random_number);
}
