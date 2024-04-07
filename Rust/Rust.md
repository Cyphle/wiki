# Rust

## Sections
* [Cargo package manager](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Cargo.md)
* [Variables, structures, etc](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_VariablesStruct.md)
* [Control Flow](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_ControlFlow.md)
* [Pattern Matching](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_PatternMatching.md)
* [Organization - Packages, crates and co](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Organization.md)
* [Ownership](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Memory.md)
* [Rust Functional](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Functional.md)
* [Rust Object](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Object.md)
* [Generic, Traits & Lifetime](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Generic.md)
* [Test](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Test.md)
* [Error Handling](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_ErrorHandling.md)
* [Smart Pointers](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_SmartPointers.md)
* [Concurrency & Parallelism](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Concurrency.md)
* [Macros](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Macros.md)
* [Unsafe Rust](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Unsafe.md)

## Links
* [Doc](https://www.rust-lang.org)
* [Other doc](https://doc.rust-lang.org/nomicon/index.html)
* [Reference](https://doc.rust-lang.org/reference/items/unions.html)
* [Book](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
* [Crates repository](https://crates.io)
* [Rust Playground](play.rust-lang.org)
* [Google Rust doc](https://google.github.io/comprehensive-rust/)

## Principles
* Rust is immutable by default. To tell a variable is mutable use `mut`
* An associated function is a function that's implemented on a type, like `String::new`
* Rust is references `&` to optimize memory. No need to create copies of objects
* A `crate` is a Rust collection source code files
* Rust allows to shadow a variable by using its name multiple times
```
let maVariable = "Coucou"

let maVariable: u32 = 12
```
* Variables are immutables
* When typing a variable can lead to multiple types, like parsing a string, have to add a type annotation `let myVar: u32 = "32".parse...`
* Rust is multi paradigm
* Rust use snake case for functions and variables as convention

## Debug
* Struct can print information when debugging. Add `#[derive(Debug)]` and use `{:?}` or `{:#?}` as placeholder
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


println!("rect1 is {:?}", rect1);
```
* There is also the macro `dbg!(&rect1);` that prints in `stderr` instead of `stdout` like `println!`
```
dbg!(&rect1);
```
* `db!` macro takes ownership and returns it

## derive attribute
* `derive` attribute which is annotated is a macro that generate default implementation of traits
    * `Debug`
    * `PartialEq`
    * `Eq`
    * etc
* https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#partialord-and-ord-for-ordering-comparisons