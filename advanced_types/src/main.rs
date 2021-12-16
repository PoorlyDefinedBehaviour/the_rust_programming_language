// Advanced types
//
// Using the newtype pattern for type safety and abstraction
//
// newtypes can be used to ensure that values are never confused
// because they are of different types.
//
//
// Usage example 1
//
// Ensuring type safety by having different types for the same
// underlying type.
//
// We can create Millimeters and Meters structs wrapped u32 values in a new type.
// If we wrote a function with a parameter of type Millimeters, we couldn't
// compile a program that accidentally tried to call that function
// with a value of type Meters or plain u32.
//
//
// Usage example 2
//
// Wrap a type and expose only the methods that we care about.
//
// Another use of the newtype pattern is abstracting away some
// implementation details of a type: the new typer can expose
// a public API that is different from the API of the private
// inner type if we used the new type directly to restrict
// the available functionality, for example.

// Creating type synonyms with type aliases
//
// Along with the newtype pattern, Rust provides the ability
// to declare a type alias to give an existing type another nane,
// For this we use the type keyword.
//
// The main use case for type synonyms is to reduce repetition
// when we have complicated types such as Box<dyn Fn() + Send + 'static>.
//
// We could create a type synonym to simplify the type usage:
//
// type Thunk = Box<dyn Fn() + Send + 'static>

type Kilometers = i32;

fn example_type_synonym() {
  // Kilometers is a synonym for i32,
  // Kilometers is not a separate new type.
  // Values that have the type Kilometers will be treated
  // the same as value of type i32.
  let x: i32 = 5;
  let y: Kilometers = 5;
  assert_eq!(10, x + y);
}

fn main() {
  example_type_synonym();
}
