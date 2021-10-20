use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1..101);

  loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Err(_) => continue,
      Ok(num) => num,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
      Ordering::Greater => println!("Too big!"),
    }
  }
}
