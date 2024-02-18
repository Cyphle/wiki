# Go Concurrency

* See godoc.org/runtime

## Coroutines & Waitgroups
* To launch a function in its own coroutine, use `go` keyword
* If nothing is done to wait for launched coroutine, program may terminate before them. We can use `WaitGroup` to wait for coroutines to finish
```
func main() {
    var wd sync.WaitGroup

    fmt.Println("OS\t\t", runtime.GOOS")
    fmt.Println("ARCH\t\t", runtime.GOARCH)
    fmt.Println("CPUs\t\t", runtime.NumCPU())
    fmt.Println("Goroutines\t", runtime.NumGoroutine())

    wg.Add(1) // One thing to wait
    go foo() // New coroutine to launch function foo
    bar()

    fmt.Println("CPUs\t\t", runtime.NumCPU())
    fmt.Println("Goroutines\t", runtime.NumGoroutine())
    wg.Wait()
}

func foo() {
    for i := 0; i < 10; i++ {
        fmt.Println("foo": i)
    }
    wg.Done()
}

func bar() {
    for i := 0; i < 10; i++ {
        fmt.Println("bar:", i)
    }
}
```
* You needn't close every channel when you've finished with it. It's only necessary to close a channel when it is important to tell the receiving goroutines that all data have been sent. A channel that the garbage collector determines to be unreachable will have its resources reclaimed whether or not it is closed.

## Shared variables
* Share variables between coroutines through channels to avoid race conditions. Do not use shared memory
* A function launched in a coroutine that returns a value, this returned value is discarded at the end. Use channel to get the value

## Mutex
* Mutex in go are instruction that wrap instructions to lock variables. It prevents other coroutines to accept the same block of code
```
func main() {
    counter := 0

    const gs = 100
    var wg sync.WaitGroup
    ws.Add(gs)

    var mu sync.Mutex

    for i := 0; i < gs; i++ {
        go func() {
            mu.Lock()
            v := counter
            runtime.Gosched()
            v++
            counter = v
            mu.Unlock()
            wd.Done()
        }()
    }
    wg.Wait()
}
```

## Atomic
* Atomic operation is an operation that cannot be split
```
func main() {
    counter := 0

    const gs = 100
    var wg sync.WaitGroup
    ws.Add(gs)

    for i := 0; i < gs; i++ {
        go func() {
            atomic.AddInt64(&counter, 1)
            runtime.Gosched()
            fmt.Println("Counter\t", atomic.LoadInt64(&counter))
            wg.Done()
        }()
    }
    wg.Wait()
}
```

## Channels
* Channels block execution
```
func main() {
    c := make(chan int)

    c <- 42 // This blocks execution

    fmt.Println(<-c)
}
```
* Use channels in coroutines
```
func main() {
    c := make(chan int)

    go func() {
        c <- 42 // Blocks in coroutine
    }()

    fmt.Println(<-c)
}
```
* Putting value in channel `c <- 42`
* Creating a channel `c := make(chan int)`
* Taking values off of a channel `<-c`
* Buffered channel `c := make(chan int, 4)`

### Directional channel
* Send only channel
```
func main() {
    c := make(chan <- int, 2) // send
    cr := make(chan<-) // Send
    cs := make(<-chan) // Receive

    c <- 42
    c <- 43

    fmt.Println(<-c) // Cannot do that as it is an only send channel
}
```
* Send and receive only
```
func main() {
    c := make(chan int)

    // Send
    go foo(c)

    // Receive
    go bar(c)

    fmt.Println("about to exit")
}

// Send
func foo(c chan<- int) { // Send only channel
    c <- 42
}

// Receive
func bar(c <-chan int) {
    fmt.Println(<-c)
}
```

### Using range
```
func main() {
    c := make(chan int)

    // Send
    go foo(c)

    // Receive
    for v := range c {
        fmt.Println(v)
    }
}

// Send
func foo(c chan<- int) { // Send only channel
    for i := 0; i < 100; i++ {
        c <- 42
    }
    close(c)
}
```

### Select
* Select is like switch case but for coroutines
```
func main() {
    even := make(chan int)
    odd := make(chan int)
    quit := make(chan int)

    // Send
    go send(even, odd, quit)

    receive(even, odd, quit)
}

func send(e, o, q chan<- int) {
    for i:= 0; i < 100; i++ {
        if i % 2 == 0 {
            e <- i
        } else {
            o <- i
        }
    }
    // close(e)
    // close(o)
    q <- 0
}

func receive(e, o, q <-chan int) {
    for {
        select {
            case v := <- e:
                fmt.Println("from the even channel:", v)
            case v := <- o:
                fmt.Println("from the odd channel:", v)
            case v := <- q:
                fmt.Println("from the quit channel:", v)
                return
        }
    }
}
```

### Comma ok idiom
* It is to get status of a channel. `ok` is to check if a channel is cloed or not. If false, channel is closed, if true it is not closed
```
func main() {
    even := make(chan int)
    odd := make(chan int)
    quit := make(chan int)

    // Send
    go send(even, odd, quit)

    receive(even, odd, quit)
}

func send(e, o, q chan<- int) {
    for i:= 0; i < 100; i++ {
        if i % 2 == 0 {
            e <- i
        } else {
            o <- i
        }
    }
    // close(e)
    // close(o)
    q <- 0
}

func receive(e, o, q <-chan int) {
    for {
        select {
            case v := <- e:
                fmt.Println("from the even channel:", v)
            case v := <- o:
                fmt.Println("from the odd channel:", v)
            case i, ok := <- q: // ok should be called active => true is active, false is closed
                if !ok {
                    fmt.Println("from comma ok", i, ok)
                    return
                } else {
                    fmt.Println("from comma ok", i)
                }
        }
    }
}
```