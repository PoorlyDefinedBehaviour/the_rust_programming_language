// Accessing or modifying a mutable static variable
//
// In Rust, global variables are called static variables.
//
// Static variables have a fixed address in memory, using the value
// will always access the same data.
//
// Static variables can store references with the 'static lifetime,
// which is ellided here.
static HELLO_WORLD: &str = "Hello, world!";

// Static variables can be mutable.
//
// Accessing and modifying mutable static variables is unsafe.
//
// With mutable data that is globally accessible, it's difficult
// to ensure there are no data races, which is why Rust considers
// mutable static variables to be unsafe.
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
  // Modifying mutable static variables is unsafe.
  unsafe {
    COUNTER += inc;
  }
}

fn main() {
  dbg!(&HELLO_WORLD);

  add_to_counter(3);

  // Accessing mutable static variables is unsafe.
  unsafe {
    dbg!(COUNTER); // 3
  }
}
