/// Function-like macros
///
/// Function-like macros define macros that look like function calls.
/// Similarly to macro_rules! macros, they're more flexible than functions;
/// for example, they can take an unknown number of arguments.
/// However, macro_rules! macros can be defined only using the match-like
/// syntax. Function-like macros take a TokenStream parameter and their
/// definition manipulates that TokenStream using rust code as the other
/// two types of procedural macros do.
///
/// An example of function-like macro is an sql! macro that might
/// be called like so:
///
/// let sql = sql!(SELECT * FROM posts WHERE id = 1);
///
/// The macro would parse the SQL statement and check that it's sintactically correct,
/// which is mure more complex processing than a macro_rules! macro can do.
///
/// The sql! macro would be defined like this:
///
/// #[proc_macro]
/// pub fn sql(input: TokenStream) -> TokenStream {
///   todo!()
/// }
///
/// The macro receiver that is between the parethenses when the macro
/// is called sql!(...) and returns a TokenStream to geneate
/// the desired code.

fn main() {
  println!("Hello, world!");
}
