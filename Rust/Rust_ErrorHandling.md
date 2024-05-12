# Rust Error Handling

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

## Question mark operator
```
use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let integer = input.parse::<i32>()?;

    println!("the value is {:?} is integer {:?}", input, integer);
    Ok(integer);
}

fn main() {

}
```
* question mark operator is to propagate error instead of handling the error in place