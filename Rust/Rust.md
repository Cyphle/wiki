# Rust

## Sections
* [Cargo package manager](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Cargo.md)
* [Variables, structures, etc](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_VariablesStruct.md)
* [Control Flow & Pattern matching](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_ControlFlow.md)
* [Organization - Packages, crates and co](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Organization.md)
* [Ownership & lifetime](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Memory.md)
* [Rust Functional](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Functional.md)
* [Rust Object](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Object.md)
* [Generic, Traits](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Generic.md)
* [Test](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Test.md)
* [Error Handling](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_ErrorHandling.md)
* [Smart Pointers](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_SmartPointers.md)
* [Concurrency & Parallelism](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Concurrency.md)
* [Macros](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Macros.md)
* [Unsafe Rust](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Unsafe.md)
* [Size](https://github.com/Cyphle/wiki/blob/main/Rust/Rust_Size.md)

## Links
* [Doc](https://www.rust-lang.org)
* [Other doc](https://doc.rust-lang.org/nomicon/index.html)
* [Reference](https://doc.rust-lang.org/reference/items/unions.html)
* [Book](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
* [Crates repository](https://crates.io)
* [Rust Playground](play.rust-lang.org)
* [Google Rust doc](https://google.github.io/comprehensive-rust/)

## Principles
* Rust is immutable by default. To tell a variable is mutable use `mut`
* An associated function is a function that's implemented on a type, like `String::new`
* Rust is references `&` to optimize memory. No need to create copies of objects
* A `crate` is a Rust collection source code files
* Rust allows to shadow a variable by using its name multiple times
```
let maVariable = "Coucou"

let maVariable: u32 = 12
```
* Variables are immutables
* When typing a variable can lead to multiple types, like parsing a string, have to add a type annotation `let myVar: u32 = "32".parse...`
* Rust is multi paradigm
* Rust use snake case for functions and variables as convention
* No overloading

## Debug
* Struct can print information when debugging. Add `#[derive(Debug)]` and use `{:?}` or `{:#?}` as placeholder
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


println!("rect1 is {:?}", rect1);
```
* There is also the macro `dbg!(&rect1);` that prints in `stderr` instead of `stdout` like `println!`
```
dbg!(&rect1);
```
* `db!` macro takes ownership and returns it

## derive attribute
* `derive` attribute which is annotated is a macro that generate default implementation of traits
    * `Debug`
    * `PartialEq`
    * `Eq`
    * etc
* https://doc.rust-lang.org/book/appendix-03-derivable-traits.html#partialord-and-ord-for-ordering-comparisons

## Some usefull patterns
### Initialization of struct instances
* Rust convention says that it is good practice to create an initialization function (new)
```
pub struct Student {
    id: u8,
    pub age: u8,
    pub name: String,
}

impl Student {
    pub fn new(std_name: String) -> Result<Self, String> {
        if std_name.chars().all(|x| matches!(x, 'a'..='z')) {
            Of(Self {
                id: 11,
                age: 20,
                name: std_name,
            })
        } else {
            Err("The name is invalid".to_string())
        }
    }
}
```

### Default constructor
```
impl Default for Student {
    fn default() -> Self {
        Self {
            id: 0,
            age: 20,
            name: "Unknown".to_string(),
        }
    }
}
```
* And use `unwrap_or_default`. Try to unwrap result if err, call default
```
let std_1 = Student::new("Joseph123".to_string()).unwrap_or_default();
println("{:?}", std_1);
```
* Or we can use macro `#[derive(Default)]`

### Builder pattern
```
#[derive(Debug, Default, Clone)]
struct Customer {
    name: String,
    username: String,
    membership: Membershiptype,
    gender: char,
    counter: String,
    age: u8,
}

#[derive(Debug, Clone)]
enum Membershiptype {
    new,
    causual,
    loyal
}

impl Default for Membershiptype {
    fn default() -> Self {
        Membershiptype::new
    }
}

impl Customer {
    fn new(name: String) -> CustomerBuilder {
        CustomerBuilder {
            name: name,
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct CustomerBuilder {
    nae: String,
    username: Option<String>,
    membership: Option<Membershiptype>,
    gender: Option<Char>,
    country: Option<String>,
    age: Option<u8>,
}

impl CustomerBuilder {
    fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    fn membership(&mut self, membership: Membershiptype) -> &mut Self {
        self.membership = Some(membership);
        self
    }

    fn gender(&mut self, gender: char) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    fn country(&mut self, country: String) -> &mut Self {
        self.country = country;
        self
    }

    fn age(&mut self, age: u8) -> &mut Self {
        self.age = age;
        self
    }

    fn build(&mut self) -> Customer {
        Customer {
            name: self.name.clone(),
            username: self.username.clone().unwrap_or_default(),
            membership: self.membership.clone().unwrap_or_default(),
            gender: self.membership.clone().unwrap_or_default(),
            country: self.country.clone().unwrap_or_default(),
            age: self.age.clone().unwrap_or_default(),
        }
    }
}
```

### Simplifying structs
* Avoid borrowing of individual fields of a struct especially when mixing by reference and by value
```
struct A {
    f1: u32,
    f2: u32,
    f3: u32,
}

fn fn1(a &mut A) -> &u32 {
    &a.f1
}

fn fn2(a &mut A) -> u32 {
    a.f1 + a.f3
}
```