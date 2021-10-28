// grep - Globally search a Regular Expression and Print
use std::{env, process};
mod grep;

use grep::{Config, ConfigError};

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    match err {
      ConfigError::NotEnoughArguments { expected, got } => {
        println!("expected {} arguments, got {}", expected, got);
        process::exit(1);
      }
    };
  });

  if let Err(e) = grep::run(config) {
    println!("Application error: {}", e);
    process::exit(1);
  }
}
