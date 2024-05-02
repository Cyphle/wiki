# Rust functional

## Functions
* Convetion for functions is snake case
* Return statement does not have semicolon
```
fn multiplication(num1: i32, num2: i32) -> {
    println!("Computing multiplication");
    num1 * num2
}
```
* To return multiple values, use tuples which can be destructured
```
fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    ...
}
```
* `format!` macro is used to format strings
```
let full_name = {
    let first_name = "John";
    let last_name = "Doe";
    format!({first_name} {last_name})
};
```

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
* Closures are anonymous functions
```
fn is_valid_user<V1, v2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
where
    V1: Fn(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    simple_validator(name) && advance_validator(age)
}

fn main() {
    let person_1 = User {
        name: String::from("Someone"),
        age: 35,
    };

    // Closure
    let validate_user_simple = |name: &str| name.len() != 0;
    let validate_user_advance = |age: u8| age >= 30;

    println!("User validity {}", is_valid_user(&person_1.name, person_1.age, validate_user_simple, validate_user_advance));
}
```
* To enforce move of parameters in closure, use `move` keyword
```
let validate_user_simple = move |name: &str| { ... }
```

## Function pointers
* Function pointers and the same as closure but don't capture variables
```
fn validate_user_simple(name: &str) -> bool {
    name.len() != 0
}

fn main() {
    let person_1 = User {
        name: String::from("John"),
        age: 35,
    }

    let validate_user_advance = |age: u8| age >= 30;

    println!(
        "User validity {}",
        &person_1.name,
        person_1.age,
        validate_user_simple, // Is now a function pointers and not a closure
        validate_user_advance
    )
}
```
* We can pass function pointers anytime instead of a closure
```
fn validate_user_simple(name: &str) -> bool {
    name.len() != 0
}

fn validate_user_advance(age: u8) -> bool {
    age >= 30
}

fn is_valid_user(
    name: &str, 
    age: u8, 
    simple_validator: fn(&str) -> bool, // fn is to say that it is a function. It is not the same as the closure trait Fn 
    advance_validator: fn(u8) -> bool
) -> bool {
    simple_validator(name) && advance_validator(age)
}

fn main() {
    let person_1 = User {
        name: String::from("John"),
        age: 35,
    }

    let validate_user_advance = |age: u8| age >= 30;

    println!(
        "User validity {}",
        &person_1.name,
        person_1.age,
        validate_user_simple, // Is now a function pointers and not a closure
        validate_user_advance
    )
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
* Example of iterator
```
struct Employee {
    name: String,
    salary: u16,
}

struct Employee_Records {
    employee_db: Vec<Employee>,
}

impl Iterator for Employee_Records {
    type item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let mut emp_1 = Employee {
        name: String::from("John"),
        salary: 40_000,
    };
    let mut emp_2 = Employee {
        name: String::from("Joseph"),
        salary: 30_000,
    };

    let mut emp_db = Employee_Records {
        employee_db: vec![emp_1, emp_2],
    }

    println("{:?}", emp_db.next());
    println("{:?}", emp_db.next());
    println("{:?}", emp_db.next());

    for employee in emp_db {
        println!("{employee}"); // Values are unwrapped in for loop
    }
}
```
* Values are unwrapped in for loop when using an iterator

### IntoIterator
* Trait InteoIterator
```
trait IntoIterator {
    type Item;
    type IntoIter: Iterator;

    fn into_iter(self) -> Self::IntoIter;
}
```
* Struct that implement this trait can be transformed into iterator
* Example
```
struct Book {
    title: String,
    author: String,
    genre: String,
}

struct BookIterator {
    properties: Vec<String>,
}

impl Iterator for BookIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.properties.is_empty() {
            Some(slef.properties.remove(0))
        } else {
            None
        }
    }
}

impl IntoIterator for Book {
    type Item = String;
    type IntoIter = BookIterator;

    fn into_iter(self) -> Self::IntoIter {
        BookIterator {
            properties: vec![self.title, self.author, self.genre],
        }
    }
}

fn main() {
    let book = Book {
        title: "Digital Image Processing".to_string(),
        author: "Gonzales".to_string(),
        genre: "Science book".to_string(),
    }

    let mut book_iterator = book.into_iter();

    println("{:?}", book_iterator.next());
    println("{:?}", book_iterator.next());
    println("{:?}", book_iterator.next());
    println("{:?}", book_iterator.next());
}
```

### Iterating over collections
```
fn main() {
    let mut vec_1 = vec![45, 30, 85, 90, 41, 39];
    
    for values in &vec_1 {
        println("{values}");
    }
}
```
* We can iterate over a hashmap
```
let mut person: HashMap<String, i32> = HashMap::new();
person.insert("Hannash".to_string(), 40);
person.insert("Joseph".to_string(), 44);
person.insert("Sara".to_string(), 55);

for (name, age) in &mut person {
    // name is immutable as it is the key of the map
    println!("The person {} has and age of {}", name, age);
}
```

### Combinators
```
fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];

    // Dumb and ugly solution
    let mut result: Vec<String> = vec![];
    for word in words {
        if word.starts_with("a") || word.starts_with("b") {
            let uppercase_word = word.to_uppercase();
            result.push(uppercase_word);
        }
    }

    // With combinator
    let result = words
        .into_iter()
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        .map((|word| word.to_uppercase()))
        .collect::<Vec<String>>(); // collect::<Vec<String>> is to help compiler to know the type wanted
}
```
* Combinators are functions to execute function over iterators (same as iterator adaptors)
* Combinators are lazy

### Iterating through Options
```
fn main() {
    let some_product = Some("laptop");
    let mut products = vecè["cellphone", "battery", "charger"];

    // Naive
    match some_product {
        Some(product) => products.push(product),
        _ => {}
    }

    // Better way
    products.extend(some_product);

    // Another way
    let products_iter = products.iter().chan(some_product.iter());
    for prod in products_iter {
        println!("{:?}", prod);
    }
}
```
* Another example
```
let products = vec![Some("charger"), Some("battery"), None, Some("cellphone")];

let prod_without_none = products
    .into_iter()
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect::<Vec<&str>>();

let prod_without_none = products.into_iter().flatten().collect::<Vec<&str>>;
```

## Advanced functions and closures
* It is possible to pass closures to function but also functions
* Function are coerce to `fn` (not the same as trait `Fn`) which is a function pointer
```
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```
* `fn` is a type and not a trait
* Function pointers implement `Fn`, `FnMut` and `FnOnce` so it is possible to pass a function pointer instead of a closure
* It is best practice to accept `fn` and closure. One use case to only accept `fn` and not closure is when calling external functions from languages that do not have closure
* Example of using closure or function pointer for the same thing
```
    // Closure
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Function pointer
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
```
* Enum name are also initializers so we can use name as function pointers
```
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```
* It is not possible to return a closure because they do not have concrete type of trait. Rust does not know the size of the returned closure
```
// This won't compile
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```
* But we can box the closure to be returned
```
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```