# Web Assembly

## What is Web Assembly
* Web Assembly is a technology that allows to compile code into near native binary that can be executed in mordern web browsers
* It allows applications to be fast
* It can be executed alongside of Javascript
* Web Assembly formats
    * The .wat text format (called wat for "WebAssembly Text") uses S-expressions, and bears some resemblance to the Lisp family of languages like Scheme and Clojure.
    * The .wasm binary format is lower-level and intended for consumption directly by wasm virtual machines. It is conceptually similar to ELF and Mach-O.
* Web Assembly is not designed to be only for the web but as of today it is mostly used in Javascript echosystems (web and NodeJS)

## Commands
* `wasm-pack build` to build the project and generate a binary and JS/TS glue files
* When in a configured project to run in web, run `npm init wasm-app www` to generate example application that uses the `.wasm`
* `wasm-pack test --chrome --headless` to run tests in WASM context. Available browsers `--safari`, `--firefox`, `--node`
* `wasm-pack login` and `wasm-pack publish` to login and publish to npm. Package name is
```
[package]
name = "wasm-game-of-life-my-username"
```

## Notes
* WebAssembly has a linear memory
    * WebAssembly has a very simple memory model. A wasm module has access to a single "linear memory", which is essentially a flat array of bytes. This memory can be grown by a multiple of the page size (64K). It cannot be shrunk.
* WebAssembly has no access to the garbage collected heap (for the moment) but Javascript can access WebAssembly memory but only as ArrayBuffer of scalar value (u8, i32, ...). So it is possible to use this to optimize calls and copies. Example
```
// From Rust expose pointers
pub fn width(&self) -> u32 {
    self.width
}

pub fn height(&self) -> u32 {
    self.height
}

pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
}

// In javascript
const cellsPtr = universe.cells();
const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
```
* WebAssembly functions also take and return scalar values. These are the building blocks from which all WebAssembly and JavaScript communication is constituted.
* `wasm_bindgen` defines a common understanding of how to work with compound structures across this boundary. It involves boxing Rust structures, and wrapping the pointer in a JavaScript class for usability, or indexing into a table of JavaScript objects from Rust. `wasm_bindgen` is very convenient, but it does not remove the need to consider our data representation, and what values and structures are passed across this boundary. Instead, think of it as a tool for implementing the interface design you choose.
* When designing an interface between WebAssembly and JavaScript, we want to optimize for the following properties:
    1. Minimizing copying into and out of the WebAssembly linear memory. Unnecessary copies impose unnecessary overhead.
    2. Minimizing serializing and deserializing. Similar to copies, serializing and deserializing also imposes overhead, and often imposes copying as well. If we can pass opaque handles to a data structure — instead of serializing it on one side, copying it into some known location in the WebAssembly linear memory, and deserializing on the other side — we can often reduce a lot of overhead. `wasm_bindgen` helps us define and work with opaque handles to JavaScript Objects or boxed Rust structures.
* Macro `#[wasm_bindgen]` is used to expose something to Javascript
* Macro `#[wasm_bindgen_test]` is used to define a test that is run in WASM context

## Tools
* `utils::set_panic_hook();` is a hook that display in the browser console errors
* `debugger;` instructions in the code can be used
* print in console
```
[dependencies.web-sys]
version = "0.3"
features = [
  "console",
]

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Usage
log!("...")
```
* `#[bench]` to create bench performances

## Resources
* Useful crates https://rustwasm.github.io/docs/book/reference/crates.html
* Tools : https://rustwasm.github.io/docs/book/reference/tools.html
* A tutorial with Rust: https://rustwasm.github.io/docs/book/game-of-life/setup.html