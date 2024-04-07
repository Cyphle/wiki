# Rust Object

* Rust has OOP features
```
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```
* There is no inheritance mechanism
* Specifying type `Box<dyn Draw>` with `dyn` keyword means that we want something that implements `Draw` trait without having to know which is the concrete type

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
* Sometimes we want to return somethings but empty. We can return unit type `()` like `JoinHandle<()>`
