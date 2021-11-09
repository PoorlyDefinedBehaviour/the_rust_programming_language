#![allow(unused)]
// Object-oriented programming is a way of modeling programs.
// Objects came from Simula in the 1960s. Those objects influenced
// Alan Kay's programming architecture in which objects pass
// messages to each other. He coined the term object-oriented
// programming in 1967 to describe this architecture.
//
// Characteristics of Object-oriented languages
//
// There is no consensus in the programming community about
// what features a language must have to be considered object
// oriented. Rust if influenced by many programming paradigms,
// including OOP.
//
// Objects contain data and behaviour
//
// The Gang of Four book defines OOP like this:
//
// Object-oriented programs are made up of objects.
// An object packages both data and the procedures
// that operate on tha data. The procedures are typically
//called methods or operations.
//
// Encapsulation that hides implementation details
//
// Another aspect commonly associated with OOP is the idea
// of encapsulation, which means that the implementation
// details of an object aren't accessible to code using
// that object. Therefore, the only way to inreact
// with an object is through its public api.

// Encapsulation example
//
// [AveragedCollection] is a list where
// the average of its elements if pre computed.
struct AveragedCollection {
  list: Vec<i32>,
  average: f64,
}

impl AveragedCollection {
  pub fn add(&mut self, value: i32) {
    self.list.push(value);

    // NOTE: This is really bad, since:
    // We don't always need the average, so having it precalculated
    // is not that useful.
    // This make [add] O(n) instead of O(1).
    self.update_average();
  }

  pub fn remove(&mut self) -> Option<i32> {
    let result = self.list.pop();

    match result {
      Some(value) => {
        // NOTE: Again, this makes [remove] O(n) instead of O(1).
        self.update_average();
        Some(value)
      }
      None => None,
    }
  }

  fn update_average(&mut self) {
    let total: i32 = self.list.iter().sum();
    self.average = total as f64 / self.list.len() as f64;
  }
}

// Inheritance as a type system and as code sharing
//
// Inheritance is a mechanism whereby an object can inherit
// from another object's definition, thus gaining the parent
// object's data and behaviour without having to define them again.
//
// One of the reasons to use inheritance relates to the type system:
// to enable a child type to be used in the same place as the parent type.
// This is also called polymorphism, which menas that you can
// substitute multiple objects for each other at runtime if they share
// certain characteristics.

fn main() {
  println!("Hello, world!");
}
