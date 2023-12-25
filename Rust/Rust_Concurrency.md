# Rust concurrency and parallelism

* Concurrent programming, where different parts of a program execute independently
* Parallel programming, where different parts of a program execute at the same time

## Threads
* Creating a new thread
```
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
* When the main thread is finished, all other threads are terminated even if they have not finished. However, it is possible to wait for a thread to finish with `JoinHandle`
```
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```
* Calling `join` blocks the current thread
* We can `move` ownership of variables to closures of threads. It is necessary when capturing variables from outside the thread
```
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // Need to take ownserhip
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

## Transfer data between threads with messages
* Rust provides channel to pass messages (as in Go)
* A channel as a transmitter and a receiver. If one of them drop the channel, then it is closed
* `mpsc::channel` is for multiple producer and single consumer
```
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // Sends message to main thread
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

//////
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```
* `send` sends `Result<T, E>` which results in error if there is no receiver
* `recv` is to receive by blocking the thread and sends `Result<T, E>` which results in error if there is no transmitter
* `try_recv` will not block and return immediatly `Result<T, E>`. Returns Ok value if there is a message available and Error if there is no message
* `send` moves the value
* To use multiple producers, use `clone`
```
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
```

## Shared state concurrency - shared memory

### Mutex<T>
* For mutual exclusion
* Rust helps to lock and unlock mutex
```
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```
* The call `lock` returns a smart pointer `MutexGuard` wrapped in `LockResult`
* It is not possible to move ownership of a mutex in multiple threads
* `Rc<T>` is not thread safe so it is not possible to use it to count references to mutex
* Use `Arc<T>` which is atomic RC and so thread safe
```
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```
* `Mutex<T>` provides interior mutability. In the eample `counter` is immutable but we mutate it with `*num += 1`
* Beware that mutex can provoque deadlocks. For instance when a thread needs to lock two ressources and two thread lock each one of the resources. They wait for each of them. See `deadlock mitigation strategies` to solve the issue

## Concurrency traits
* Trait `Send` allows to transfer ownership between threads. Almost all types implement `Send` except for some like `Rc<T>`
* Trait `Sync` allows to be referenced by multiple threads. Almost all types are sync
* Manually implementing `Send` and `Sync` is unsafe