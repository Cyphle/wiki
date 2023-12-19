# Cargo, the package manager

* Cargo is Rust build system and package manager
* `cargo build --release` builds the project. Release option is to create a release
* `cargo run` builds and run the project
* `cargo check` check if program is compilable
* `cargo new <project_name>` new cargo project
* `cargo update` updates dependencies
* `cargo doc --open` generates a doc of dependencies of current project
* `cargo test` to run tests

## Profiles
* Cargo has two main profiles `dev` with `cargo build` and `release` with `cargo build --release`
* To customize settings, use `Cargo.toml`
```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3 // optimization level. the more is the more optimization and build time
```

## Doc
* Documentation comments are with `///` and will generate HTML documentation by running `cargo doc`
* Doc uses markdown
* Adding example in comments for doc are used as tests
* Use `pub use` to abstract internal organization and export elements at higher level
```
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

## Publishing crates
* Create an account on crates.io
* Login with `crate login <key>`

## Metadata
* Add metadata to crates in .toml file
```
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

## Publishing
* Publish with `cargo publish`

## Workspaces
* Cargo has workspaces
* A workspace is a set of packages sharing the same `Cargo.lock` file and output directory
* https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

## Installing binaries
* Use `cargo install <binary>` to install a binary. They are usually tools.

## Extending cargo
* If a binary is named `cargo-something` and is in `$PATH`, it is usable as `cargo something`