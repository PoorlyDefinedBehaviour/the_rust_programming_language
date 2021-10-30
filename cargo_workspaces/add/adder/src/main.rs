// A workspace is a set of packages that share the same Cargo.lock
// and output directory.
//
// If two packages depend on the same external dependency,
// cargo will resolve the dependency to the same version
// because there is a single Cargo.lock at the workspace
// root. The point here is to make the packages compatible.
use add_one;

fn main() {
  let num = 10;

  println!(
    "Hello, world! {} plus one is {}!",
    num,
    add_one::add_one(num)
  );
}
