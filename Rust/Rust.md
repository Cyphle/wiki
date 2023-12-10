# Rust

## Links
* [Doc](https://www.rust-lang.org)
* [Book](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
* [Crates repository](https://crates.io)

## Cargo, the package manager
* Cargo is Rust build system and package manager
* `cargo build --release` builds the project. Release option is to create a release
* `cargo run` builds and run the project
* `cargo check` check if program is compilable
* `cargo new <project_name>` new cargo project
* `cargo update` updates dependencies
* `cargo doc --open` generates a doc of dependencies of current project

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

## Error handling
* Process that can make the program to panic but not necessary terminate can return a result
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
* Then we can react to whatever the result is
```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```
* A shortcut for `match` on error is `unwrap` which returns the value if OK or call `panic!` if not.
* Similarly, `expect` is used to specify an error message when in `panic!`. `expect` convey more intent than `unwrap` so is preferable
* To propagate the error, just return it
```
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```
* A shortcut to propagate error is `?`, so a shorter code is
```
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```
* Or shorter
```
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
```
* `?` can also be used with `Option`
```
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```
