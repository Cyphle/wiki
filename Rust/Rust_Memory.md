# Memory management

## Ownership
Memory is managed through a system of ownership with a set of rules that the compiler checks

### Ownership rules
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

### Stack & heap
It is important to understand stack and heap with Rust
* Stack is First In Last Out (FILO). Add data on stack must have known fixed size
* Data with unknown size must be in heap. When allocating on the heap, it returns a pointer which can be stored in stack
* Allocating on heap is more expensive than pushing on the stack
* Calling a function puts things on the stack including local variables

### Moving values
* Moved values is when an object (not primitive) is referenced by another variable. Then the pointer is hold by the second variable and the first one cannot be used. It is to avoid freeing memory twice (for each variable). It is a shallow copy, or a move
```
let s1 = String::from("hello");
let s2 = s1; // Value of s1 has been moved to s2.

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

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

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
* Primitives are not moved because there is no pointer, they are stored on the stack. But they are passed by value, meaning we cannot change value in a function
```
fn main() {
    let my_thing = 42;
    on_the_stack(my_thing);
    println!("After on the stack but in main {my_thing}"); // Prints 42
}

fn on_the_stack(mut something: i32) {
    something = 54;
    println!("Something in on_the_stack {something}"); // Prints 54
}
```

### References & borrowing
* Borrowing is establishing a reference to some data. Just like pointers with some rules. Does not take ownership.
* Rules:
    * At any time, you can have either one mutable reference OR any number of immutable references
    * References must always be valid
* When using reference to pass variables, we don't need to return the value because the function only borrows the reference. We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.
```
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
```
* References, using `&`, are immutable so we cannot change the value of a reference
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
* To create a mutable reference, the variable must be mutable
* Rust compiler ensures that there is no dangling reference
* Mutable references when assigned are moved, while immutable references are copied

### Dereferencing
* It is the act to get the value of a reference
```
fn main() {
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let deref_copy = *ref_1; // deref_copy gets the value of ref_1 which is 42
    *ref_1 = 13;
    println!("some_data is: {some_data}, deref_copy is: {deref_copy}"); // Prints 13 and 42

    let mut heap_data = vec![5, 6, 7];
    let ref_1 = &mut heap_data;
    let deref_copy = *ref_1; // Does not work because vec is on the heap and dereferencing means changing ownership.
}
```

## Lifetime
* Lifetimes ensure that references are valid as long as we need them to be
* Every reference has a lifetime. Most of the time lifetimes are implicit and inferred like types.
* A lifetime is `'a` between `<>` and for each parameter that needs to be associated to lifetime. Example:
```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
It means: the returned reference will be valid as long as both the parameters are valid. This is the relationship between lifetimes of the parameters and the return value. We’ll name the lifetime `'a` and then add it to each reference.

This example is because if we have
```
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
then it won't compile because the compiler, never the programmer, does not know whether the function will return a reference to x or to y. It is not possible to know which reference will be valid at the end of function.
* Use of function with lifetime definition
```
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```
In this example, string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. Run this code, and you’ll see that the borrow checker approves; it will compile and print The longest string is long string is long.
* Lifetime definition tells that it takes the smallest lifetime. This will not compile
```
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```
`string2` is not valid anymore in the outer scope so result cannot have lifetime long enough. The explanation is that the compiler cannot know that `string1` is the longest and so possibly `result` is a borrowed reference to `string2` which has a smaller lifetime and thus does not exist anymore for `println!`
* Lifetime definition does not have to be on all parameters
```
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```
as function always returns `x`, we do not need to tell that the lifetime can be the one of `y`. The lifetime of `y` does not have any relationship with the lifetime of `x` nor the return value.
* If struct contains references, it has to define lifetime
```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```
this works because `novel` does not go out of scope before `ImportExcerpt` which holds a reference a slice.
* For methods of struct
```
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```
* the lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query)
```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

### Lifetime elision
* Lifetime elision are some use cases integrated into the compiler that do not need lifetime. It is because the compiler knows how the references behave
```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
* In earlier versions, this code would have needed lifetime. We do not need to follow what common patterns are integrated to avoid specifying lifetimes.
* To determine if lifetime annotations are needed, the compiler tries to apply the three following rules:
    * The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
    * The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
    * The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
* If, after applying these rules, some input or output lifetimes could not be determined, then we need to specify lifetime annotations.

### Static lifetime
* They are lifetime that can live for the entire duration of the program `let s: &'static str = "I have a static lifetime.";`
