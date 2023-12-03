# Rust base principles - Structs, loops, etc

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
* String literal are on stack because size can be determined at compile time
* String type are on heap because size cannot be determined at compile time

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

## Struct
* Declare
```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
* Use
```
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```
* Or mutable struct
```
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```
* It is possible to use shorthand when field name and parameter are the same (same behaviour as javascript)
```
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```
* Destructuring is possible. `..user1` has to come last to tell that remaining fields are from `user1`
```
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```
* Struct can have unnamed fields. They are called `tuple struct`
```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```
* Struct can have no field. They are called unit like struct
```
fn main() {
    let subject = AlwaysEqual;
}
```

### Methods
* Methods are functions that are declared inside a struct and receive `self` as first parameter (like in Python)
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
* 'Static' methods that don't have `self` as first parameter are called with `::`
```
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

Rectangle::square(32);
```

## Enums
* Enum can have receive data
```
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```
* Received data don't have to be of the same type
```
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
```

## Option
* Option is the concept that something can be present or sent (same as either). Like if you access an element of a collection or an empty collection
* Rust does not have the null concept but uses option
```
enum Option<T> {
    None,
    Some(T),
}
```

## match, pattern matching
* Rust `when` or `swatch case` equivalent is `match`.
* For default values, if needed to catch the value, use `other`. If not, use `_`
```
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // Or _ => () if nothing has to be done
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```

### if let
* `if let` control flow is usefull for matching only on value of an enum
```
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
```
* Is it also possible to add else
```
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Equivalent
        let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```