// Using threads to run code simultaneously
//
// In most current operating systems, an execute program's code is run
// in a process, and the operating system manages multiple processes
// at once. Within your program, you can also have independent
// parts that run simultaneously. The features that run these
// independent parts are called threads.
//
// Splitting the computation in your program into multiple
// threads can improve performance at the cost of complexity.
//
// Because threads can run simultaneously, there's no inherent
// guarantee abou the order in which parts of your code
// on different threads will run. This can lead to problems, such as:
//
// Race conditions, where threads are accessing data or resources
// in inconsistent order.
//
// Deadlocks, where two threads are waiting for each other to finish
// using a resource the other thread has, preventing both threads
// from continuing.
//
// Bugs that happen only in certain situations and
// are hard to reproduce an fix reliably
//
// Many programming languages implement threads in a few different ways.
// Many operating systems provide an API for creating new threads.
// This model where a language calls the operating system APIS to
// create threads is sometimes called 1:1, meaning one operating
// system thread per one language thread.
//
// Many programming languages provide their own special implementation
// of threads.
// Programming language-provided threads are known as green threads,
// and languages that use these threads will execute them in the
// context of a different number of operating systems threads.
// For this reason, the green-threaded model is called the M:N model:
// there are M green threads per N operating system threads, where M
// and N are not necessarily the same number.
//
// Each model has its own advantages and trade-offs, and the trade-off most
// important to Rust is runtime support. Runtime is a confusing term
// and can have different meanings in different contexts.
//
// In this context, by runtime we mean code that is included
// by the language in every binary.
// This code can be large or small depending on the language,
// but every non-assembly language will have some amount of runtime code.
// For that reason, colloquially when people say a language has no
// runtime, they often mean small runtime.
// Smaller runtimes have fewer features but have the advantage
// of resulting in smaller binaries, which make it easier
// to combine the language with other languages in more contexts.
// Rust needs to have nearly no runtime and cannot compromise
// on being able to call into C to maintain performance.
//
// The green-threading M:N model requires a larger language runtime
// to manage threads. As such, the Rust standard libarry only provides
// an implementation of 1:1 threading.

fn main() {
  println!("Hello, world!");
}
