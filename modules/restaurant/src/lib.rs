#![allow(unused)]
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    // We can use super to access items in the parent module
    super::serve_order();
  }

  fn cook_order() {}
}
