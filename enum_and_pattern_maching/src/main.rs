// Enums allow you to define a type by
// enumerating its possible variants.

// Any IP address can be either a version four of version six address,
// but not both at the same time.
// That property of IP addresses makes the enum data structure
// appropriate, because enum values can only be one of its variants.
#[derive(Debug)]
enum MyIpAddr {
  // Enum variants may contain data
  V4(String),
  V6(String),
}

fn route(_: &MyIpAddr) {

}

fn main() {
  let home = MyIpAddr::V4(String::from("127.0.0.1"));

  dbg!(&home); // V4("127.0.0.1")

  let loopback = MyIpAddr::V6(String::from("::1"));

  dbg!(&loopback); // V6("::1")

  // MyIpAddr::* are all of the same type: MyIpAddr
  route(&home);
  route(&loopback);

  // Turns out, wanting to store IP address and encode
  // which kind they are is so common that the standard library
  // has a definition we can use.
  // Here's how the standard library defines it:
  #[derive(Debug)]
  struct Ipv4Addr {
    // ...
  }

  #[derive(Debug)]
  struct Ipv6Addr {
    // ...
  }

  #[derive(Debug)]
  enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
  }

  dbg!(IpAddr::V4(Ipv4Addr{}));
  dbg!(IpAddr::V6(Ipv6Addr{}));

  // ---
  #[derive(Debug)]
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  // we can have impl blocks for enums aswell
  impl Message {
    fn call(&self) {
      // ...
    }
  }

  let _ = Message::Write(String::from("hello"));
  let _ = Message::Move { x: 1, y: 3 };
  let _ = Message::ChangeColor(0, 0, 0);
  let message = Message::Quit;

  message.call()
}
