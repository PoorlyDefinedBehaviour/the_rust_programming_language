use multithreaded_web_server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
/// The two main protocols involved in web servers are the
/// Hypertext Transfer Protocol(HTTP) and the
/// Transmission Control Protocol(TCP).
///
/// Both protocols are request-response protocols,
/// meaning a client initiates requests and a server listens
/// to the requests and provides a response to the client.
/// The contents of those requests and responses are defined by
/// the protocols.
///
/// TCP is the lower-level protocol that describe the details of
/// how information gets from one server to another but doesn't
/// specify what that information is.
///
/// HTTP builds on top of TCP by defining the contents of the requests
/// and responses. It's technically possible to use HTTP with other protocols,
/// but in the vast majority of cases, HTTP sends its data over TCP.

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 1024];

  stream.read(&mut buffer).unwrap();

  let get = b"GET / HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", "hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "404.html")
  };

  let contents = fs::read_to_string(filename).unwrap();

  let response = format!(
    "{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    contents.len(),
    contents
  );

  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
