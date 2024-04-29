# Rust automated tests

## Example
* Example of test
```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```
* Run tests with `cargo test`. Command accept a a string to filter tests containing the string for filtering run tests `cargo test add`
* It is possible to add custom fail messages
```
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
```
* To test that the program should panic
```
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
```
* It is also possible to use result
```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```
* `cargo test` has several options like defining number of threads used to run tests `cargo test -- --test-threads=1`
* To show standard output when running tests `cargo test -- --show-output`
* To run ignored tests `cargo test -- --ignored`
```
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```
* To run all including ignored `cargo test -- --include-ignored`
* With Rust, it is possible to test private function
```
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```
* Hierarchy for tests
```
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests // for integration tests
    └── integration_test.rs
```
* Annotation `#[cfg(test)]` is used when tests are written in same file as code to tell the cargo that they are tests

## Assert a function panic
```
mod shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {

        }
    }
}


#[test]
#[should_panic]
fn should_create_and_panic() {
    let some_circle = shapes::Circle::new(-11.0);
}
```

## Controlling test execution
* To run tests containing a string `cargo test <My_string>`
* To run `#[ignore]` tests `cargo test --ignore`

## Integration tests
* Place them in a directory in source `tests`
```
/
    tests
```
* Files are named `xxx_test.rs`
* Example
```
// In tests/order_test.rs
use mypackage::{Category, Customer, Order, Product};

#[test]
fn test_total() {
    let product = Product::new(1, String::from("Book"), 19.9, Category::Books);
    let customer = Customer::new(1, String::from("Bob"), String::from("bob@bob.fr"));
    let order = Order::new(2, product, customer, 3);

    assert_eq!(format!("{:2}", order.total_bill(), "65.67"));
}
```
* To run only tests in `tests` directory `cargo test --test <mytestfile>`

## Benchmark testing
* Rust tools are in test
* Use Criterion tool
```
// In Cargo.toml
[dev-dependencies]
criterion = "0.4.0"

[[bench]]
name = "sorting_benchmark"
harness = false
```
* In a folder `benches` under the root folder, files `xxx_benchmark.rs`
```
use learning_rust::{sort_algo_1, sort_algo_2};
use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers: Vec<i32> vec![1, 2, 3, 55, 4, 32];

    c.brench_function("Sorting Algorithm", |b| {
        b.iter(|| sort_algo_1(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);
```