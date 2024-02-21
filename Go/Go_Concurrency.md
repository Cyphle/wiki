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

## Fan in
* Get value from multiple channels and merge into one
```
func receive(even, odd <-chan int, fanint chan<- int) {
    var wg.sync.WaitGroup
    wg.Add(2)

    go func() {
        for v := range even {
            fanin <- v
        }
        wg.Done()
    }()

    go func() {
        for v := range odd {
            fanin <- v
        }
    }()

    wg.Wait()
    close(fanin)
}
```
```
func main() {
    c := fanIn(boring("Joe"), boring("Ann"))
    for i := 0; i < 10; i++ {
        fmt.Println(<-c)
    }
    fmt.Println("You're both boring; I'm leaving")
}

func boring(msg string) <-chan string {
    c := make(chan string)
    go func() {
        for i := 0; ; i++ {
            c <- fmt.Sprintf("%s %d", msg, i)
            time.Sleep(time.Duratin(rand.Intn(1e3)) * time.Millisecond)
        }
    }()
    return c
}

func fanIn(input1, input2 <-chan string) <-chan string {
    c := make(chan string)
    go func() {
        for {
            c <- <-input1
        }
    }()
    go func() {
        for {
            c <- <-input2
        }
    }()
    return c
}
```

## Fan out
* Send value to multiple channels
```
func fanOutIn(c1, c2 chan int) {
    var wg sync.WaitGroup
    for v := range c1 {
        wg.Add(1)
        go func(v2 int) { // Create multiple channels => fan out
            c2 <- timeConsumingWork(v2)
            wg.Done()
        }(v)
    }
    wg.Wait()
    close(c2)
}

func timeConsumingWork(n int) int {
    time.Sleep(time.Microsecond * time.Duration(rand.Intn(500)))
    return n + rand.Intn(1000)
}
```

## Context
* Use context to avoid leaking coroutines. For example if we launch a process that launches multiple coroutines, when the process is stopped, we have to stop coroutines
* Go check official documentation
```
func main() {
    ctx := context.Background()

    fmt.Println("context:\t", ctx)
    fmt.Println("context err:\t", ctx.Err())
    fmt.Println("context type:\t%T\n", ctx)
    fmt.Println("----")

    ctx, cancel = context.WitchCancel(ctx)

    fmt.Println("context:\t", ctx)
    fmt.Println("context err:\t", ctx.Err())
    fmt.Println("context type:\t%T\n", ctx)
    fmt.Println("----")

    cancel()

    fmt.Println("context:\t", ctx)
    fmt.Println("context err:\t", ctx.Err())
    fmt.Println("context type:\t%T\n", ctx)
    fmt.Println("----")
}
```
* How to use
```
func main() {
    ctx, cancel := context.WithCancel(context.Background())

    fmt.Println("error check 1:", ctx.Err())
    fmt.Println("num gortins 1:", runtime.NumGoroutine())

    go func() {
        n := 0
        for {
            select {
                case <-ctx.Done():
                    return // Return to not leak coroutine
                default:
                    n++
                    time.Sleep(time.Millisecond * 200)
                    fmt.Println("working", n)
            }
        }
    }()

    time.Sleep(time.Second * 2)
    fmt.Println("error check 2:", ctx.Err())
    fmt.Println("num gortins 2:", runtime.NumGoroutine())

    fmt.Println("about to cancel context")
    cancel()
    fmt.Println("cancelled context")

    time.Sleep(time.Second * 2)
    fmt.Println("error check 3:", ctx.Err())
    fmt.Println("num gortins 3:", runtime.NumGoroutine())
}
```