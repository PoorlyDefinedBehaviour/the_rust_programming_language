use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("guess must be between 1 and 100, got {}", value);
    }

    Self { value }
  }

  pub fn value(&self) -> i32 {
    self.value
  }
}

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..101);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line");

    let guess = match guess.trim().parse::<i32>() {
      Err(_) => continue,
      Ok(num) => Guess::new(num),
    };

    println!("You guessed: {}", guess.value());

    match guess.value().cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
      Ordering::Greater => println!("Too big!"),
    }
  }
}
