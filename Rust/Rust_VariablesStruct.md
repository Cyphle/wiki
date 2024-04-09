# Rust base principles - Structs, loops, etc

## Variables, constants and structs
* Variables are declared with `let`. There is type inference
* Constants are declared with `const`
* Type conversion
```
let a: i32 = 10
let b: f64 = a as f64
```
* `let unit = ();` is the unit type

## Compound types
* Tuples can hold variables of multiple types. They have a fix size, once declare, cannot change size. `let tup: (i32, f64, u8) = (500, 6.4, 1);`. Can spread `let (x, y, z) = tup`
* Arrays can hold variables of same type. They have a fix length. Can be initialize directly or specified with type and length. `let months = ["Janvier", "Février"]; let a: [i32, 5] = [1, 2, 3, 4, 5];`

## Arrays
* Arrays `let mut array_1 = [4, 5, 6, 8]`. `[0; 10]` creates an array of 10 elements that are all equal to 0. Size is immutable.
* Arrays can hold variables of same type. They have a fix length. Can be initialize directly or specified with type and length. `let months = ["Janvier", "Février"]; let a: [i32, 5] = [1, 2, 3, 4, 5];`

## Vectors
* Vectors `let vec: Vec<i32> = vec![4, 5, 6, 8, 9]`. It can change size.

## Tuples
* Tuples can hold different types of values. `let my_info = ("Salary", 4000);`. Access element with `my_info.1`. 
* Tuples can hold variables of multiple types. They have a fix size, once declare, cannot change size. `let tup: (i32, f64, u8) = (500, 6.4, 1);`. Can spread `let (x, y, z) = tup`
* Can declare tuple structs `struct Point_2D(i32, i32)`
```
struct Point_2D(i32, i32);

let point1 = Point_2D(1, 3);
```

## Static
* Static variable `static WELCOME: &str = "Welcome to Rust";`. Constantes are inlined and statics uses a given memory location.
```
static WELCOME: &str = "Welcome to Rust";

let a = WELCOME; // Same memory location as b
let b = WELCOME; // Same memory location as a

const PI = 3.14;
let x = PI; // Not same memory location as y
let y = PI; // Not same memory location as x
```

### String
* String slice with `&str` and `String`
* `let s = "Hello world"` is a string literal or string slice. It is immutable.
* `let s = String::from("Hello world")` is a string type | object. It can grow in size for example with `push` method
* String literal and string object works differently. For instance
```
fn my_function(s: &String) // Accept String type and not literal

fn my_function(s: $str) // Accept String literal and String type and String slice
```
* String literal are on stack because size can be determined at compile time
* String type are on heap because size cannot be determined at compile time
* String literal `println!("{s}")`
* String have length and capacity.

### Slices
* Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
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
* Beware of ownership in structs!
```
struct Car {
    owner: String,
    year: u8,
    fuel_level: f32,
    price: u32
}

fn main() {
    let mut my_car {
        owner: String::from("John doe"),
        year: 2010,
        fuel_level: 0.0,
        price: 5_000
    }

    let extracted_owner = my_car.owner; // Field owner has been moved
    println!("Owner {}", my_car.owner); // will not work because has been moved (partial move)
}
```
* Unit struct has no field `struct ABC;`

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
* Enum can receive methods
```
enum TravelType {
    Car(f32),
    Train(f32),
    Aeroplane(f32)
}

impl TravelType {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelType::Car(miles) => miles * 2.0,
            TravelType::Train(miles) => miles * 3.0,
            TravelType::Aeroplane(miles) => miles * 5.0,
        };
        allowance
    }
}

fn main() {
    let participant = TravelType::Car(60.0);

    println!("Allowance of participant is: {}", participant.travel_allowance());
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
* Usage example
```
struct Student {
    name: String,
    grade: Option<u32>
}

fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        if student.name == *student_name {
            return student.grade;
        }
    }
    None
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(95)
        },
        Student {
            name: String::from("Bob"),
            grade: Some(87)
        },
        Student {
            name: String::from("Charlie"),
            grade: None
        }
    ];

    let student_name = String::from("Bob");
    let student_grade = get_grade(&student_name, &student_db);

    match student_grade {
        Some(grade) => println("Grade is: {grade}"),
        None => println("No grade")
    }
}
```

## Result
* Result structure of Rust allows to represent a result or an error
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```
fn check_student_get_grade(student_name: &String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found"));
}

let student_status = check_student_get_grade(&student_name, &student_db);

match student_status {
    Ok(grade) => {
        if let Some(grade) = option_grade {
            println("Grade is: {grade}");
        }
    },
    Err(error_msg) => println!("{error_msg}"),
}
```

## Type alias
* Rust allows for type alias
```
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;
```

## Never type
* The never type is to tell a function will never return
```
fn bar() -> ! {
    // --snip--
}
```
* For instance `continue` has a never return type. When used, functions coerce to other type. Like this will return u32
```
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```
* Other examples are `panic!` and `loop`
```
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

    print!("forever ");

    loop {
        print!("and ever ");
    }
```

## Dynamically sized type
* If Rust cannot know the size of a type at compile time but need runtime, we cannot create something like 
```
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
```
Because `str` does not have a know size. A solution is to use `&str` instead of `str`.
* The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.
* We can combine str with all kinds of pointers: for example, `Box<str>` or `Rc<str>`.
* Every trait is a dynamically sized type we can refer to by using the name of the trait.
* we mentioned that to use traits as trait objects, we must put them behind a pointer, such as `&dyn Trait` or `Box<dyn Trait>` (`Rc<dyn Trait>` would work too).
* DSTs : Dynamically sized type
* Rust automatically implement trait `Sized` for everything whose size is known at compile time
```
fn generic<T>(t: T) {
    // --snip--
}
// is in fact
fn generic<T: Sized>(t: T) {
    // --snip--
}
```
* If we don't know the size at compile time, we have the special syntax for only `Sized` trait
```
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```
If size is not known with `?` syntax, use `&T` instead of `T`