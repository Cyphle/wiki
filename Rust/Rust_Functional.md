# Rust functional

## Closures
* Rust allows to pass closures as arguments of functions
* An example of using closure is `unwrap_or_else` which accepts a closure as argument. The closure is evaluated latter if needed.
* Example of closure
```
let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```
* Closure usually don't need to specify type as it can be infered
* Different possible definitions
```
fn  add_one_v1   (x: u32) -> u32 { x + 1 } // Function
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // Closure
let add_one_v3 = |x|             { x + 1 }; // Closure
let add_one_v4 = |x|               x + 1  ; // Closure
```
* If no type are specified for closure, the first use make the inference and 'lock' the type of the closure. For instance, 
```
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5); => This will produce error as it was used with a String
```
* A closure can
    * Immutably borrow
    * Mutably borrow
    * Take ownership
* Closure implement one or more
    1. FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
    2. FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
    3. Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
* Example
```
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

## Iterators
* All iterators implement the trait
```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
* Method that call next of an iterator are called `consuming adaptors`
* `Iterator adaptors` are methods defined on the iterator that don't consume the iterator. For example map:
```
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
```