# Rust organizations - Packages, crates, etc

## Overview
* Packages: A Cargo feature that lets you build, test, and share crates
* Crates: A tree of modules that produces a library or executable
* Modules and use: Let you control the organization, scope, and privacy of paths
* Paths: A way of naming an item, such as a struct, function, or module

## Crate
* A crate is the samellest amount of code the Rust compiler considers at a time.
* Binary crate: is a crate that can be compiled and executed. It must have a main function
* Library crate: they don't have main and define functionalities usable by other crate

## Organizing code
* Library does not have `main.rs` but `lib.rs` and they do not have `main` method
* The crate root is the source that Rust starts from and is the root module
* A package is a bundle of n crates and has a `Cargo.toml` file that describes how to build those crates
* `use` keyword brings a path into scope

## Rules
* Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
* Declaring modules: In the crate root file, you can declare new modules; say, you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
    * Inline, within curly brackets that replace the semicolon following mod garden
    * In the file src/garden.rs
    * In the file src/garden/mod.rs
* Declaring submodules: In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
    * Inline, directly following mod vegetables, within curly brackets instead of the semicolon
    * In the file src/garden/vegetables.rs
    * In the file src/garden/vegetables/mod.rs
* Paths to code in modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.
Private vs public: Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.
* The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus to make use of that type in the scope.

## Example
```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs

// src/main.rs
use crate::garden::vegetables::Asparagus;

pub mod garden; // Can use content of file src/garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

// src/garden.rs
pub mod vegetables; // can use content of file src/garden/vegetables.rs because file is named garden

// src/garden/vegetables.rs
pub struct Asparagus {}
```