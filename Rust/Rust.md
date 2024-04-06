# Rust

## Links
* [Doc](https://www.rust-lang.org)
* [Other doc](https://doc.rust-lang.org/nomicon/index.html)
* [Reference](https://doc.rust-lang.org/reference/items/unions.html)
* [Book](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
* [Crates repository](https://crates.io)

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

## Macros
* Calls with `!`. Example: `println!`. If call function `println`

## derive attribute
* `derive` attribute which is annotated is a macro that generate default implementation of traits
    * `Debug`
    * `PartialEq`
    * `Eq`
    * etc
* https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#partialord-and-ord-for-ordering-comparisons