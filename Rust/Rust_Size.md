# Size in Rust

* A type is sized if its size can be determined at compile time
* A type is unsized if its size cannot be determined at compile time
* All primitives are sized
```
use std::mem::size_of;

fn main() {
    // Sized types
    println!("i32 size is: {}", size.of::<i32>());

    println("Box<i32> is: {}", size_of::<Box<i32>>());
    println("fn(i32) -> i32 is: {}", size_of::<fn(i32)-> i32>());

    // Unsized types
    println!("[i32] size is: {}", size_of::<[i32]>()); // Error: cannot calculate size of slice. Can have size of reference which is 1 word (8 bits)
    let a: [i32, 3]; // Fixed size array
    println("str size is: {}", size_of::<str>());
}
```

## Sized trait
* Auto trait are implemented by default
```
let x: i32 = Default::default();
```
* To remove default implementation, we can use `negative_impl` library
```
use negative_impl::negative_impl;

struct ABC;

#[negative_impl]
impl !Send for ABC {}
```
* But we cannot remove implementation of `Size` trait that allow to calculate size of a struct which is the sum of all its fields + some space
* To enforce using struct that have size calculable
```
fn some_fn<T: ?Sized>(t: T) {} // ? is relaxed sized bound: ? is only used to relax the implicit Sized trait bound for type parameters or associated types. ?Sized may not be used as a bound for other types.
```
* Some struct cannot be used when size cannot be calculated at compile time like struct with array.
```
struct UnsizedStruct {
    sized_field_1: i32,
    unsized_field: [i32],
}

fn main() {
    let x = UnsizedStruct {
        sized_field_1: 3,
        unsized_field: [3],
    }
}
```
* An unsized struct must
    1. Must have a single unsized field
    2. The unsized field must be the last
* To solve the issue, we can use generic parameter with the optionally sized marker `?Sized`
```
struct UnsizedStruct<T: ?Sized> {
    sized_field_1: i32,
    unsized_field: T,
}

fn main() {
    let x = UnsizedStruct {
        sized_field_1: 3,
        unsized_field: [3],
    };
}
```

## Unsized coercion
* Deref coercion is when a type can be used instead another
```
fn str_slice_fn(s: &str) {}

fn array_slice_fn<T>(s: &[T]) {}

fn main() {
    let some_string = String::from("String");
    str_slice_fn(&stome_string); // String can be coerce into str

    let slice: &[i32] = &[1];
    let vec = vec![1];
    let array = [1, 2, 3];

    array_slice_fn(slice);
    array_slice_fn(&vec); // Vec can be coerced into array (deref coercion)
    array_slice_fn(&array); // Unsized coercion
}
```
* Unsized coercion: The array `array` which has a known size can be used in a function expecting something with unknown size
```
trait Some_Trait {
    fn method(&self);
}

impl<T> Some_Trait for [T] {
    fn method(&self) {}
}

fn main() {
    let slice: &[i32] = &[1];
    let vec = vec![1];
    let array = [1, 2, 3];

    slice.method();
    vec.method(); // deref coercion
    array.method(); // Unsized coercion
}
```

## Zero sized types
### Never type
* Never type is `!`
```
#![feature(never_type)] // Never type is in nightly build and activated with this macro (may be in current release now)

fn unrecoverable_state() -> ! {
    panic("This function will never return normally with something valid");
}

fn main() {
    let x = unrecoverable_state();
    let x: !;

    let x = match "123".parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!(),
    }
}
```
* Using never type in function return
```
fn function() -> Result<i32, !> {} // Not allowed
fn function() -> Result<!, i32> {} // Allowed
```
* Never type is
    - Computation that never oroduces a value
    - Function returning never, will never returns normally
    - No associated value, and can be coerced to all types

### Unit type
* Unit type means a lake of meaningful data or information and is `()`
```
fn f1() {}

fn division_status(divident: f64, divisor: f64) -> Result<(), String> {
    let answer = match divisor {
        0.0 => Err(String::from("Error: Division by zero")),
        _ => {
            println!("The division is invalid");
            Ok(())
        }
    };
    answer
}

fn main() {
    let y = f1(); // Stores ()

    let z = println!("Hello, world!");
    let mut vec: Vec<()> = Vec::with_capacity(0);
    vec.push(());
    vec.push(());
    vec.push(());
    assert_eq!(vec.len(), 3);
    println!("{}", vec.capacity()); // == a lot. Allocate to the highest value
}
```
* Unit type is
    - Computation returning no meaningful value
    - Function returning unit always returns normally
    - Single value, which can not be coerced