// Implementing an object-oriented design pattern
//
// The state pattern is an object-oriented design pattern.
// The crux of the pattern if that a value has some internal state,
// which is represented by a set of state objects, and the value's
// behaviour changes based on the internal state.
// The state objects share functionality.
//
// NOTE: can we replace the state pattern with type state?

trait State {
  // We use Box<Self> here because self is a trait,
  // and it's size is not known, but the size of a
  // Box<T> always is known.
  fn request_review(self: Box<Self>) -> Box<dyn State>;

  fn approve(self: Box<Self>) -> Box<dyn State>;

  // The return value will live as long as [post].
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    ""
  }
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

struct PendingReview {}

impl State for PendingReview {
  // This is an example of why this pattern is not very good
  // if implemented in an OOP way,
  // it clearly makes no sense for PendingReview to have a method
  // called request_review but it has to have it because
  // every state is implementing the same trait.
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Self {
    Self {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn content(&self) -> &str {
    // Calling Option::as_ref because we want
    // a reference to the value inside the Option.
    // &Option<T>::as_ref returns Option<&T>.
    self.state.as_ref().unwrap().content(self)
  }

  pub fn approve(&mut self) {
    // Option::take returns the Option and
    // leaves None in its place.
    if let Some(state) = self.state.take() {
      self.state = Some(state.approve())
    }
  }

  pub fn request_review(&mut self) {
    // Option::take returns the Option and
    // leaves None in its place.
    if let Some(state) = self.state.take() {
      self.state = Some(state.request_review())
    }
  }
}

fn main() {
  let mut post = Post::new();

  post.add_text("I ate a salad for lunch today");

  // Why are we able to call a method while the post
  // is in the draft state?
  // Seems like type state is coming.
  assert_eq!("", post.content());

  post.request_review();

  assert_eq!("", post.content());

  post.approve();

  assert_eq!("I ate a salad for lunch today", post.content());
}
