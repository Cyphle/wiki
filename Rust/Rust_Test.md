# Rust automated tests

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