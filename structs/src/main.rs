// A struct is a custom data type that lets you name
// and package together multiple related values that make
// up a meaningful group.

// Tuple structs
//
// They behave like tuples,
// except that each of them has their own unique type
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

// Unit-like structs
//
// Structs that have no fields are called unit-like strcuts
// because they bhave similarly to (), the unit type.
// Unit-like structs can be useful when you need to implement
// a trait on some type but don't have any data that
// you want to store in the type itself.
#[derive(Debug)]
struct S;

fn main() {
  // The entire instance must be mutable
  // if we want to mutate one of its fields
  let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

  dbg!(&user1);

  dbg!(&user1.email); // someone@example.com

  user1.email = String::from("anotheremail@example.com");

  dbg!(&user1.email); // anotheremail@example.com

  // We can create a new struct from an existing one
  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };

  dbg!(&user2);

  let black = Color(0, 0, 0);

  dbg!(black); // Color(0, 0, 0)

  let origin = Point(0, 0, 0);

  dbg!(origin); // Point(0, 0, 0)

  dbg!(S); // S
}
