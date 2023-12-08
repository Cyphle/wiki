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