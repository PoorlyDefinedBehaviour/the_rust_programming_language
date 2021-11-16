// Using extern functions to call external code
//
// Sometimes, you Rust code might need to interact with code
// written in another language. For this, Rust kas a keyword,
// extern, that facilitates the creation and use of a
// Foreign Function Interface(FFI). An FFI is a way for a programming
// language to define functions and enable a different(foreign)
// programming language to call those functions.

// Calling functions from other languages
//
// We want the call to abs function from C.
// We use extern "C" to define which application binary
// interface(ABI) the external function uses:
// the ABI defines how to call the function at the assembly level.
extern "C" {
  fn abs(input: i32) -> i32;
}

// Calling Rust functions from other languages
//
// We can also use extern to create an interface that allows
// other languages to call Rust functions.
//
// Making a Rust function callable from C.
//
// #[no_mangle] tells the Rust compiler not to mangle the function name.
//
// Mangling is when a compiler changes the name weéve given a function
// to a different name that contains more information for other
// parts of the compilalation process to consume.
//
// Every programming language mangles names differently, so for a
// Rust function to be nameable by other languages, we must disable
// the Rust compiler's name mangling.
#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("just called a Rust function from C");
}

fn main() {
  // Extern functions are always unsafe to call,
  // because Rust has no way to hold invariants with
  // code that isn't from Rust.
  unsafe {
    println!("abs(-3) = {}", abs(-3)); // abs(-3) = 3
  }
}
