# Rust control flow

## If let else
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
```
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```
* `if let` can create shadowed values `if let Ok(age) = age` but cannot be used to combine things like `if let Ok(age) = age && age > 30`. It is important to note that it is a shadowed value with same name.

## Loops
* loop creates an infinite loop
```
loop {
    // Your code here
}
```
* To exit a loop, we can use `break`. And to exit outer loop of nested loops, we can use naming syntax
```
'outer: loop {
    println!("Simple loop");
    break 'outer;
}
```
* We can assign the break value
```
let a = loop {
    break 5;
};
```

### For
* `for x in y`, x is the pattern
```
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let vec = vec![45, 46];
    for i in vec {
        println!("{i}");
    }
```

## While
```
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
```
```
    let mut num = 0;
    while num < 10 {
        num = num + 1;
    }
```

## Let
```
let PATTERN = EXPRESSION;

let x = 5;
```
* It is pattern matching

## Function parameters
* Function parameters are also patterns
```
fn foo(x: i32) { // x is a pattern
    // code goes here
}
```

## Match, pattern matching
* Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple
* General syntax
```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}

match x {
    None => None,
    Some(i) => Some(i + 1),
}

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
```
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
* Exhaustivity in arms of pattern matching is mandatory
```
let marks = 95;
let mut grade = 'N';

match marks {
    90..=100 => grade = 'A',
    80..=89 => grade = 'B',
    70..=79 => grade = 'C',
    _ => grade = 'F',
}
```
* Have to match all possible cases.
* The particular pattern _ will match anything

### Match guard
* A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen like `if x % 2 == 0`. Variable created in match guard can be used in arm
```
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
```
* We can use a variable outside a match without shadowing it if we do not use `Some`. Here `y` is `let y = 10;` and not a shadowed variable
```
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}
```
* It is possible to use multiple conditions
```
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // if x = 4 or 5 or 6 AND y is true
        _ => println!("no"),
    }
```
* We can create other variable, without shadowing, with `@` when testing arms. Here we create `id_variable` when testing `id` is in range `3..=7` (=7 in range means included)
```
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => { // In this arm, id is not a variable usable in arm
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
```

## Refutability and irrefutability
* A pattern that match everything is irrefutable like `let x = 5;` and cannot fail
* A pattern that do not match everything is refutable like `if let Some(x) = a_value` and can fail. If `a_value` is `None` it fails.
* `let` and `for` loops accept irrefutable patterns, `if let` and `while` accept both but compiler will warn to have a failure fallback
```
// This work
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

// This will be a warning. if let x is irrefutable and will always match
    if let x = 5 {
        println!("{}", x);
    };
```

## Pattern for destructuring
```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

// or

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

// or
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```
* Mixing with match
```
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```
* Mixing with enums
```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
```
* All together
```
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
```

## Ignoring values
```
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```
* Ignoring part of values
```
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
```
* To declare a variable that is not use and tell Rust not to give warning, use `_` to start the name of the variable
```
fn main() {
    let _x = 5;
    let y = 10;
}
```
* `_` does not bind but `_<variable>` bind. So `_<variable>` takes ownership
```
    let s = Some(String::from("Hello!"));

    // This does not work because _s takes ownership
    if let Some(_s) = s {
        println!("found a string");
    }

    // We use it here but we took ownership
    println!("{:?}", s);

    // This works
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
```

## Ignoring rest
* With `..` we can ignore what we did not specified
```
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    fn main() {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }
```
* `..` must be unambiguous. This won't work
```
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```
