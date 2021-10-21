fn main() {
  // The if let syntax lets you combine if and let into a less
  // verbose way to handle values that match one pattern
  // while ignoring the rest

  let some_u8_value = Some(0u8);

  // This match expression coud be replaced with a if let
  match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
  }

  // Same as above but cleaner
  if let Some(3) = some_u8_value {
    println!("three");
  }

  // We can also add a else branch to an if let
  if let Some(3) = some_u8_value {
    println!("three");
  } else {
    println!("not three");
  }
}
