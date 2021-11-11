// Encoding states and behaviour as types
//
// We will encode states and state transaitions
// into different types. Consequently, Rust's type checking
// system will prevent attempts to use draft posts
// where only published posts are allowed by issuing
// a compiler error.
//
//             request_review            approve
//                  │                       │
//                  │                       │
// ┌─────────────┐  │  ┌─────────────────┐  │ ┌────────┐
// │             │  │  │                 │  │ │        │
// │  DraftPost  ├──┴──►PendingReviewPost├──┴─►  Post  │
// │             │     │                 │    │        │
// └──────┬──────┘     └────────┬────────┘    └────┬───┘
//        │                     │                  │
//        │                     │                  │
//    add_text               approve            content
//    request_review
struct Post {
  content: String,
}

impl Post {
  // [new] returns DraftPost because
  // every post starts in the Draft state.
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

// NOTE: I bet this will evolve to a generic type parameter
// in Post with different impl blocks depending on the type of
// the generict type parameter.
struct DraftPost {
  content: String,
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }
}

// Every post state has duplicated fields.
struct PendingReviewPost {
  content: String,
}

impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}

fn main() {
  let mut post = Post::new();

  post.add_text("I ate salad for lunch today");

  // This should and does not compile because
  // a post in the Draft state has no content
  // to be asserted on.
  // assert_eq!("", post.content()); -- compiler error, method does not exist.

  let post = post.request_review();

  // Only approved posts have the content method.
  let post = post.approve();

  assert_eq!("I ate salad for lunch today", post.content());
}
