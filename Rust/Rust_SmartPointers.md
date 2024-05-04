# Smart pointers

* Smart pointers are pointers that are data structures that act like pointers and have special capabilities and metadata like `reference counting` which is a pointer that allows for multiple owners of a data and when no pointer point to the data clean up
* References borrow data, smart pointers own data. For instance `String` and `Vec<T>`
* Smart pointers implement traits 
    * `Deref` to allow to behave like references
    * `Drop` that allow to customize behavior when pointer goes out of scope

## Box<T>: Box smart pointer
* `Box<T>` is a pointer that stores data on the heap
* Use cases
    * When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    * When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    * When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
* Basic usage
```
fn main() {
    let b = Box::new(5);
    println!("b = {}", b); // Prints b = 5
}
```
* An example of use case is recursive type, when a type contain something of the same type. The compiler cannot know the size because it can be infinitely recursive
```
// The following won't compile. List has infinite size
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

// This will compile as Box<List> will be a pointer to the heap and not something on the stack anymore
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```
* Rist has to know which size of memory a return type needs. We can't have a function which returns a trait like `Animal`. Instead we can return a `Box` to tell that we have a pointer on the stack the points to an object on the heap that size is dynamically calculed from type `Box<dyn Animal>`
* Some examples of using box
```
struct Huge_Data;
struct Small_Data;

trait Storage {}

impl Storage for Huge_Data {}

impl Storage for Small_Data {}

fn main() {
    let data_1 = Huge_Data;
    let data_2 = Box::new(Huge_Data);

    let data_3 = data_1;
    let data_4 = data_2; // Only the box pointer is copied when moving ownership of Box

    let data_5 = Box::new(Small_Data);

    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5]; // : Vec<Box<dyn Storage>> is to allow having Box of Huge_Data and Box of Small_Data
}
```

## Deref trait
* Implementing `Deref` trait allows to customize behavior of the dereferencing operator `*`
```
fn main() {
    let x = 5;
    let y = &x; // Reference

    assert_eq!(5, x);
    assert_eq!(5, *y); // Dereference operator to get the value
}

fn main() {
    let x = 5;
    let y = Box::new(x); // This works the same as a reference even if y is an instance of Box pointing to a copied value of x and not a reference pointing to x

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
* Creating our own `Box` with `Deref`
```
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
* Deref coercion is a mechanism of Rust that allows to convert a reference from one type implementing `Deref` to another
```
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Rust calls: MyBox::deref -> &String (which implements Deref) -> String::deref -> &str
}
```
* Rust does deref coercion when it finds types and trait implementations in three cases:
    * From &T to &U when T: Deref<Target=U>
    * From &mut T to &mut U when T: DerefMut<Target=U>
    * From &mut T to &U when T: Deref<Target=U>

## Drop trait
* `Drop` trait can be implemented to define behavior when a value goes out of scope. It is usefull for example to release resources likes files or network connections
```
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
```
* It is not possible to call `drop` manually. If needed, for example to free a lock before going out of scope, call `std::mem::drop`

## Rc<T> - Reference counted smart pointer
* An example of use case is for graph. A node that has multiple edge pointing to it has virtually multiple owners and do not have to be dropped until all pointers are cleaned
* `Rc<T>` is only for single threaded and only immutable references
* Example
```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
} // This won't work because b and c share ownership of a. a has been moved to b

// Solution, use Rc
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```
* `Rc::clone` does not copy the data but increase the number of references
* To check number of references
```
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```
* Note that this smart pointer is for read only share of a value
* `Rc<T>` count `strong_count` which is the count of strong reference to clean up. Theses references share ownership. It is possible to have weak references that don't share ownership and won't be counted for cleaning. Call `Rc::downgrade` which returns a `Weak<T>` which is a smart pointer. It increases the `weak_count`.
* To check if value still exists with a weak reference, use `upgrade` which returns `Option<Rc<T>>`

## RefCell<T> and the Interior Mutability Pattern
* This pattern is to mutate data when there are immutable references to it. We have to use `unsafe` code. It tells the compiler that we will manually check the rules
* With `Box<T>` borrowing rules are at compile time but with `RefCell<T>` rules are at runtime so if a rule is broken, the program will go in panic
* `RefCell<T>` is for single threaded
* Example of use case is with mock objects
```
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message)); // As we have &self which is immutable, it would not have been possible to not use RefCell
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```
* Only one mutable borrow at the same time or the program will go in panic
```
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }
```
* To have multiple mutable borrow at the same time combine with `Rc<T>`
```
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

## Reference cycle causing memory leak
* It is possible to create reference cycle which can leak memory because values will never be dereferenced.
* One way to avoid reference cycle, use weak references of `Rc<T>`
* Example of creating a tree with strong and weak references of `Rc<T>` and `RefCell<T>`
```
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// To check references count
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```