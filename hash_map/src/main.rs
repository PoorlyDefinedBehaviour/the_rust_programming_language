// The type HashMap<K, V> stores a mapping of keys
// of type K to values of type V.
//
// It does this via a hashing function, which determines how
// it places these keys and values into memory.
//
// By default, HashMap uses a hashing function called SipHash
// that can provide resistance to Denial of Service attacks
// involving hash tables.
use std::collections::HashMap;

fn main() {
  // Just like vectors, hash maps store their data on the heap.
  let mut scores = HashMap::new();

  scores.insert(String::from("blue"), 10);
  scores.insert(String::from("yellow"), 50);

  dbg!(&scores);

  // Hash Maps and Ownership
  // Types that implement the Copy trait, like i32, will be copied
  // into the hash map.
  // Owned values like String will be moved in to the hash map.

  dbg!(&scores.get(&String::from("blue"))); // Some(10)
  dbg!(&scores.get(&String::from("unknown"))); // None

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }
}
