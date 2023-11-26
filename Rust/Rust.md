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

## Base concepts
* Variables are immutables
* When typing a variable can lead to multiple types, like parsing a string, have to add a type annotation `let myVar: u32 = "32".parse...`
* Compound types
    * Tuples can hold variables of multiple types. They have a fix size, once declare, cannot change size. `let tup: (i32, f64, u8) = (500, 6.4, 1);`. Can spread `let (x, y, z) = tup`
    * Arrays can hold variables of same type. They have a fix length. Can be initialize directly or specified with type and length. `let months = ["Janvier", "Février"]; let a: [i32, 5] = [1, 2, 3, 4, 5];`
* Rust is multi paradigm
* Rust use snake case for functions and variables as convention

## Loops
* loop creates an infinite loop
```
loop {
    // Your code here
}
```

## String
* `let s = "Hello world"` is a string literal
* `let s = String::from("Hello world")` is a string type | object
* String literal and string object works differently. For instance
```
fn my_function(s: &String) // Accept String type and not literal

fn my_function(s: $str) // Accept String literal and String type and String slice
```

## Memory management - Ownership
* Memory is managed through a system of ownership with a set of rules that the compiler checks
* It is important to understand stack and heap with Rust
    * Stack is First In Last Out (FILO). Add data on stack must have known fixed size
    * Data with unknown size must be in heap. When allocating on the heap, it returns a pointer which can be stored in stack
    * Allocating on heap is more expensive than pushing on the stack
    * Calling a function puts things on the stack including local variables
* Ownership rules
    * Each value in Rust has an owner.
    * There can only be one owner at a time.
    * When the owner goes out of scope, the value will be dropped.
* Moved values is when an object (not primitive) is referenced by another variable. Then the pointer is hold by the second variable and the first one cannot be used. It is to avoid freeing memory twice (for each variable). It is a shallow copy, or a move
```
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); <= throws an error
```
* A type that implements the `Copy` trait does not move. It only does deep copy
* A function receiving an object moves the variable into the function
```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```
* When using function, be careful to return values if needed to use it after because it has been moved. Or use references
```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
* Use references to avoid move. `calculate_length` does not have ownership of s1
```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
* References are immutable so we cannot change the value of a reference
```
fn main() {
    let s = String::from("hello");

    change(&s); <== throws an error
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```
* But it is possible to use mutable references, it is done like variables
```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
* It is not possible to create multiple references of a mutable reference
```
  let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; <== throws an error

    println!("{}, {}", r1, r2);
```
* To make multiple references of a mutable reference, use block scopes
* It is not possible to have immutable and mutable references in the same scope
* There are implicit scopes. So it is possible to have immutable and mutable references if some references are not used anymore
```
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```
* Rust compiler ensures that there is no dangling reference

## Slices
* A slice contains reference to a part of a string
```
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```
* It works with ASCII. For UTF8, to not store part of a character, see https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
* With slice, it is not possible to change the original string
```
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```
* It is also possible to create slices of other objects
```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

## Macros
* Calls with `!`. Example: `println!`. If call function `println`