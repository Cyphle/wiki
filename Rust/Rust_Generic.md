# Rust Generics, Traits and Lifetime

## Generics
* Function
```
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```
* Struct
```
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```
* Defining method such as
```
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
in generic types (without <T>), means that only `Point<f32>` will have the method
* It is possible to mix generics
```
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

## Traits
* Traits are shared behaviors. A trait defines functionality a particular type has and can share with other types. Traits are similar to a feature often called interfaces in other languages, although with some differences.
* Defining a trait
```
pub trait Summary {
    fn summarize(&self) -> String;
}
```
* And implementing it
```
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
* Then use it
```
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```
* A restriction is that we can implement a trait only if the trait or the type is local to the crate. For example, we can implment `Display` of standard library in `Tweet` which is local but can't implement `Display` on `Vec<T>` which are both outside of crate.
* We can define default behavior of traits
```
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```
* A trait can call other methods of the trait
```
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```
* To use a trait
```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
or
```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
* If a function requires multiple trait as parameters
```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```
it is possible to write it as
```
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```
* Or use it as return type
```
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```
* But returning a trait cannot work if we want to return multiple types (like This OR That)
```
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```
* It is possible to conditionally implements a trait if the trat implements another trait. Like structs that implement `ToString` only if implements `Display`
```
impl<T: Display> ToString for T {
    // --snip--
}
```

### Advanced trait
* Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation. That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.
```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```
* Difference between generic `<T>` and associated type is that with generic we can have multiple implementation but with associated type only one
```
pub trait Iterator<T> { // can have multiple implementations like impl Iterator<String> for Counter
    fn next(&mut self) -> Option<T>;
}
```
* Trait can have default concrete type `<PlaceholderType=ConcreteType>`
```
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```
* If we implement multiple methods with same name, as for example implementing multiple traits with same method name and an own method of a struct, we can specify by fully qualifying where the method comes from. If not, the default is the own method before the traits
```
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```
* When functions are not method, they do not specify `&self` as parameter, we have to fully qualify if there are multiple implementations `<Type as Trait>::function(receiver_if_method, next_arg, ...);`
```
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```
* We can specify supertraits like `OutlinePrint: Display` to tell that to implement `OutlinePrint` we need to implement `Display`
* The rule that says we can implement a trait only if type or trait is local to the crate can be get around with `newtype pattern`. We have to create a new type that wraps the type which will implement the trait
```
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
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

## Lifetime elision
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

## Static lifetime
* They are lifetime that can live for the entire duration of the program `let s: &'static str = "I have a static lifetime.";`

## Mixing
* Traits, generic and lifetime at the same time
```
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
