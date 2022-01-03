/// Attribute-like macros
///
/// Attribute-like macros are similar to custom derive macros,
/// but instead of generating code the the deriver attribute,
/// they allow you to create new attributes.
/// They're also more flexible:
/// derive only works for struct and enums; attributes can be applied
/// to other items as well, such as functions.
///
/// Say you have an attribute named route for
/// creating routes in a web framework
///
/// #[route(GET, "/")]
/// fn index() {
///   todo!()
/// }
///
/// The #[route] attribute would be defined by the framework
/// as a procedural macro. The signature of the macro definition
/// would looks like this:
///
/// #[proc_macro_attribute]
/// pub fn route(attr: TokenStream, item: TokenStream) -> TokenSream {
///   todo!()
/// }
///
/// where attr is GET, "/" and item is the index function.
///
/// Other than that, attribute-like macros wotk the same way as custom drive
/// macros: you create a crate with the proc-macro crate type and implement
/// a function that generates the code you want.

fn main() {
  println!("Hello, world!");
}
