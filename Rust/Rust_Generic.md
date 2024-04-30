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

let origin = Point { x: 0, y: 0 };
let p1 = Point { x: 1.0, y: 4.0 };

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

let origin = Point::new(0, 0);
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
* Specialization
```
impl Point<i32, i32> {
    fn printing(&self) {
        println!("The values of the coordinates are {}, {}", self.x, self.y);
    }
}
```
* Using generics in functions
```
fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point(<T, U>) {

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
* Another notation to use a trait
```
fn shape_properties(object: T) 
where 
    T: Shape + OtherTrait + SomeOtherTrait,
{
    ...
}
```

### Trait bounds
* Use to define what to use in functions `<T: Shape>`. It is a trait bound
```
fn shape_properties<T: Shape>(object: T) {
    object.area();
    object.perimeter();
}
```
* Trait bounds can be added `<T: Shape + AnotherTrait>`. Meaning that struct has to implement both traits
* Returning a trait
```
fn returns_shape() -> impl Shape {
    let sq = Square {
        side: 5.0,
        line_width: 5,
        color: String::from("Red"),
    };
    sq
}
```

### Super trait
* It is to extend traits
```
trait Draw {
    fn draw_object(&self);
}

trait Shape: Draw { // Can also be Shape: Draw + AnotherTrait
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented, returning dummy value");
        0.0
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        ...
    }
}
```

### Trait object
* When using trait, specialized functions are generated by the compiler. It is static dispatch. We can create dynamic dispatch trait that are generated at runtime (at compile time it is more optimized). To do so, for instance `fn shape_properties_dynamic(object: Box<dyn Shape>)`. It is the `dyn` keyword that tells the compiler that it is a dynamic dispatch
* Dynamic dispatch is used when type cannot be determined at compile time like when returning different concrete types
```
fn returns_shape(dimension: Vec<f32>) -> Box<dyn Shape> {
    if dimension.len() == 1 {
        let sq = Square {
            side: dimension[0],
            line_width: 5,
            color: String::from("Red"),
        }
        Box::new(sq)
    } else {
        let rect = Rectangle {
            length: dimension[0],
            width: dimension[1],
            line_width: 5,
            color: String::from("Red"),
        }
        Box::new(rect)
    }
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
