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
* `thread::sleep(<Duration>)` stop the current thread for duration
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

## Multiple thread and ownership
```
use std::thread;

fn main() {
    let mut thread_vec: Vec<JoinHandle<()>> = vec![];
    for i in 0..10 {
        // Closure may outlive the current function and borrow i
        thread_vec.push(thread::spawn(|| {
            println!("Thread number {}", i);
        }))
    };
}
```
* To transfer ownership in a thread, use `move` keyword
```
use std::thread;

fn main() {
    let mut thread_vec: Vec<JoinHandle<()>> = vec![];
    for i in 0..10 {
        thread_vec.push(thread::spawn(move || {
            println!("Thread number {}", i);
        }))
    };

    for i in thread_vec {
        i.join();
    }
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

## Shared state concurrency
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
* The call `lock` returns a smart pointer `MutexGuard` wrapped in `LockResult`. When MutexGuard goes out of scope, it unlocks the mutex
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

## Synchronization with barriers
* A barrier enables multiple threads to synchronize at some point of computation
* When all thread reach a given point, the program can continue
```
use std::thread;
use std::sync::{Arc, Barrier};

fn main() {
    let mut threads = Vec::new();
    let barrier = Arc::new(Barrier::new(5)); // Will allow 5 threads to be blocked

    for i in 0..10 {
        let barrier = barrier.clone();
        let t = thread::spawn(move || {
            println!("Before wait {}", i);
            barrier.wait(); // Block all 5 threads until all threads meet at this point of code
            println("After wait {}", i);
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}
```

## Scoped thread
```
use std::thread;

fn main() {
    let mut vec = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|some_scope: &scope| {
        some_scope.spawn(|| {
            println!("I am first thread in the scope");
            println("{:?}", vec);
        });

        some_scope.spawn(|| {
            println!("I am second thread in the scope");
            x += 45;
        });
    });

    println!("The threads are now complete");
    vec.push(5);
    println("x: {} and vec {:?}", x, vec);
}
```
* `thread::scope` ensure that the threads spawned inside the closure ends in the scope

## Thread parking, yielding
* The park will block execution of current thread until it is unparked
```
use std::thread;
use std:time::Duration;

fn main() {
    let job_1 = thread::spawn(|| {
        println!("-- Job 1 has started --");
        println!("Waiting for job 2 to complete");
        thread::park();

        println!("-- Job 1 resued --");
        println!("-- Job 1 finished--");
    });

    let job_2 = thread::spawn(|| {
        println!("-- Job 2 started --");
        println!("-- Job 2 finished --");
    });
    job_2.join().unwrap();
    println!("Job 2 is now completed");
    println!("Job 1 will now resume");
    job_1.thread().unpark();
    job_1.join().unwrap();
}
```
* `park_timeout(Duration::from_secs(2))` park during given timeout if no unpark is called
* `thread::yield_now()` it gives up its operating system time so OS can schedule another thread

## Async Await
```
async fn printing() {
    println!("I am async function");
}

fn main() {
    let x: impl Future<Output = ()> = printing();
}
```
* `async` returns a variable implementing `Future` trait
* Futures are like promises
* `async` have to be polled or waited for result
```
let x = printing().await;
```
* But `await` gives error. We need a poller with libraries like `tokio` (in Cargo.toml [dependencies] tokio = {version = "1.17', features = ["full"]})
```
async fn printing() {
    println!("I am async function");
}

#[tokio::main]
async fn main() {
    let x: impl Future<Output = ()> = printing().await;
}
```
* Cancel a future with `cancel` method or `drop(future)`

### Tasks
* Tasks or non blocking (they are part of tokio library)
```
#[tokio::main]
async fn main() {
    let mut handles = vec![];
    println!("This code is not part of the async block");
    let s1 = String::from("Huge computation function");
    let s2 = String::from("Simpler computation function");
    let aw1 = tokio::spawn(async move {
        huge_computation(s1).await;
    });
    handles.push(aw1);

    let aw2 = tokio::spawn(async move {
        simpler_computation(s2).await;
    });
    handles.push(aw2);

    for i in handles {
        i.await.unwrap();
    }
    println!("All tasks are now completed");
}

async fn huge_computation(s: String) {
    println!("{:?} has started", s);
    for i in 0..100_000_000 {

    }
    println!("{:?} is now completed", s);
}

async fn simpler_computation(s: String) {
    println!("{:?} has started", s);
    println!("{:?} is now completed", s);
}
```

### Select
* Tokio select is to select an async function with pattern matching and execute branch of method that ends first
```
tokio::select! {
        _ = aw1 => function_1() => println!("Function 1 is completed first"),
        _ = aw2 => function_2() => println!("Function 2 is completed first"),
    };
```
```
use tokio::select;

#[tokio::main]
async fn main() {
    // Pattern matching and select one of the functions and cancel the other
    // tokio::select! {
    //     _ = function_1() => println!("Function 1 is completed first"),
    //     _ = function_2() => println!("Function 2 is completed first"),
    // };

    let aw1 = tokio::spawn(async move {
        function_1().await;
    });

    let aw2 = tokio::spawn(async move {
        function_2().await;
    });

    tokio::select! {
        _ = aw1 => function_1() => println!("Function 1 is completed first"),
        _ = aw2 => function_2() => println!("Function 2 is completed first"),
    };
}

async fn function_1() {
    println!("Function 1 has started");
    for i in 0..100_000_000 {

    }
    println!("Function 1 has ended");
}

async fn function_2() {
    println!("Function 2 has started");
    println!("Function 2 has ended");
}
```
* `join` macro ensures that functions are executed
```
tokio::join {
    function_1(),
    function_2()
}
```

## Concurrency traits
* Trait `Send` allows to transfer ownership between threads. Almost all types implement `Send` except for some like `Rc<T>`
* Trait `Sync` allows to be referenced by multiple threads. Almost all types are sync
* Manually implementing `Send` and `Sync` is unsafe