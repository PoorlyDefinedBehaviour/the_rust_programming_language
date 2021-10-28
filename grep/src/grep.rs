// Error is a trait object.
//
// We will return a type that implements the Error trait
// when something goes wrong. This way we don't need to specify
// what particular type the return value will be, this gives
// us flexibility to return error values that may be of different
// types in different errors cases.
//
//// The dyn keywords is short for dynamic.
use std::{env, error::Error, fs};

// We are using 'a here because the data returned
// by this function will live as long as contents.
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|&line| line.contains(query))
    .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();

  contents
    .lines()
    .filter(|&line| line.to_lowercase().contains(&query))
    .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let lines = if config.case_insensitive {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in lines {
    println!("{}", line);
  }

  Ok(())
}

#[derive(Debug)]
pub enum ConfigError {
  NotEnoughArguments { expected: usize, got: usize },
}

#[derive(Debug)]
pub struct Config {
  query: String,
  filename: String,
  case_insensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Self, ConfigError> {
    if args.len() < 3 {
      return Err(ConfigError::NotEnoughArguments {
        expected: 2,
        got: args.len() - 1,
      });
    }

    let query = args[1].clone();
    let filename = args[2].clone();
    let case_insensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Self {
      query,
      filename,
      case_insensitive,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";

    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "RuSt";

    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
