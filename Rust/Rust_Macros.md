# Rust macros

* Macros are a family of features in Rust:
    * declarative macros with `macro_rules!`
    * procedural macros
        * Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
        * Attribute-like macros that define custom attributes usable on any item
        * Function-like macros that look like function calls but operate on the tokens specified as their argument
* Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming
* A function has to define its signature and the number of arguments. A macro can take variable number of arguments
```
println!("hello");
println!("hello {}", name);
```
* Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.
* Writing macro is more difficult as a macro is writtent in Rust to write Rust
* You have to bring into scope macros before using it contrary of function
* Calls with `!`. Example: `println!`. If call function `println`

## Declarative macros
* To define a macro, use `macro_rules!`
* Example is `vec!` which is a macro `let v: Vec<u32> = vec![1, 2, 3];`
```
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```
* `#[macro_export]` is to tell that every crate that bring the macro into scope can use it
* Declaring a macro name is without `!`
* `( $( $x:expr ),* )` is an arm pattern. The macro has only one arm. If there a pattern match then the code after `=>` will be emitted
* Pattern matching in macro are against Rust and not values so it different. For list of pattern matching of macro: [macro pattern matching](https://doc.rust-lang.org/reference/macros-by-example.html)
* First `()` to encompass the pattern
* Second we have `$` which declares a variable for the pattern
* Third we have `()` that captures the value
* Fourth we have the pattern `$x:expr` which matches any Rust expression and gives the expression the name `$x`
* Fifth we have `,` which indicates that a separator `,` can appear to have multiple captured values
* Sixth we have `*` that tells we can have 0 or more captures preceding the `*`
* So `vec![1, 2, 3];` matches three times `$x: expr`
* `temp_vec.push()` within `$()*` is called for each captured matched pattern. So the generated code with `vec![1, 2, 3];` is
```
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

## Procudural macros
* Procedural macros act more like functions
* Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do
* The three kinds of procedural macros are:
    * custom derive, 
    * attribute-like, 
    * and function-like, 
and all work in a similar fashion.
```
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```
* `some_attribute` is to define which type of macro we are defining

### Custom Derive macros
* Procedural macro need to be on their own crate
* A derive macro allows to generate code that implatement a trait without specifying the implementation on every type that need it. It is to get a default implementation.
```
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
```
We have trait `HelloMacro` that defines a function `hello_macro` with default implementation. `HelloMacro` is defined as a `derive macro`. The macro will display `Hello, Macro! My name is TypeName!`. We can't have trait because Rust does not have reflexion. (The trait need to know the inner of its implementation to know what is `TypeName`)
* The macro is
```
pub trait HelloMacro {
    fn hello_macro();
}
```
* As macro has to on their own crate, we need to use `cargo new hello_macro_derive --lib` inside the project (for this example. we can also publish the macro independantly)
* The macro
```
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```
* `syn::parse` gives
```
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```
* As macro must return `TokenStream` and not `Result`, they have to panic if something goes wrong. That's why the `unwrap`
* TokenStream represent the parameters of the macro, in the example the struct `Pancakes`
* Derive macros only work with structs and enums

### Attribute like macros
* Attribute like macros work the same as `derive` macro but instead of generating code, they allow to create attributes
* Attribute like macros works with more things like functions
```
#[route(GET, "/")]
fn index() {
```
* The signature of the maro
```
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```
* The first parameter `attr: TokenStream` is the macro parameter `GET, "/"`
* The second parameter `item: TokenStream` is the element `fn index() {}`

### Function like macros
* Function-like macros define macros that look like function calls
* Example `let sql = sql!(SELECT * FROM posts WHERE id=1);`. The `sql!` macro wille parse the SQL statement
* The macro signature will be
```
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```