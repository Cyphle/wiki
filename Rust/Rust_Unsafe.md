# Unsafe Rust

* Unsafe Rust gives more power to code things and tell Rust that we know what we are doing. Unsafe does not disable borrow checker but allow to do unsafe actions
* Use keyword `unsafe` before block of code
* Actions that can be done
    * Dereference a raw pointer
    * Call an unsafe function or method
    * Access or modify a mutable static variable
    * Implement an unsafe trait
    * Access fields of unions
* It is a good practice to encapsulate unsafe code and provide safe API

## Raw pointers
* Raw pointers can be mutable `*mut T` or immutable `*const T`. `*` is not the dereferencing operator but part of declaration
    * In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.
* Raw pointers
    * Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    * Aren’t guaranteed to point to valid memory
    * Are allowed to be null
    * Don’t implement any automatic cleanup
* Raw pointers can be created in safe code but cannot be dereferenced outside unsafe code
```
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
```

## Calling unsafe function or method
* Unsafe function have `unsafe` keyword before declaration.
```
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
```
* Example of need for unsafe calls is when we want to split a vec in two mutable parts
```
// Does not compile
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..]) // Cannot borrow twice
}

// Use unsafe calls
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

## External function call
* Rust allows to call functions coded in another language with `extern` keyword that facilitates the creation and use of a Foreign Function Interface (FFI)
* Calls to extern functions is always unsafe because Rust cannot guarantee external code
```
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
* The "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level. The "C" ABI is the most common and follows the C programming language’s ABI
* We can define functions that can be called by other languages. We have to tell Rust compiler not to mangle name with `#[no_mangle]`. Every compiler can modify names (action of mangling)
```
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

## Accessing or modifying static variable
* A global variable in Rust is called a static variable
```
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```
* A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory. Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
* A static variable can be mutated by unsafe code
```
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```
* Static variables are accessible eveerywhere which can possibly cause data races which is why Rust requires unsafe mutation

## Unsafe trait
* We can use unsafe to implement an unsafe trait. A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
```
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```
* If we want to implement `Send` or `Sync` manually we have to tell Rust that it is `unsafe`

## Accessing fields of union
* The final action that works only with unsafe is accessing fields of a union. A union is similar to a struct, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code. Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.
* https://doc.rust-lang.org/reference/items/unions.html
