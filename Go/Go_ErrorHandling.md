# Go Error Handling

* Go does not have exception!
* Error has type `error`

## Use of error
```
func main() {
    n, err := os.Create("names.txt")
    if err != nil {
        fmt.Println(err)
        return
    }
    defer f.Close()

    r := strings.NewReader("James Bond")

    io.Copy(f, r)
}
```

## Panic
* `panic(err)` stops normal execution

## Recover
* Recover is a built-in function that regains control of a panicking goroutine. Recover is only useful inside deferred functions. During normal execution, a call to recover will return nil and have no other effect. If the current goroutine is panicking, a call to recover will capture the value given to panic and resume normal execution.
* https://go.dev/blog/defer-panic-and-recover
```
func main() {
    f()
    fmt.Println("Returned normally from f.")
}

func f() {
    defer func() {
        if r := recover(); r != nil {
            fmt.Println("Recovered in f", r)
        }
    }()
    fmt.Println("Calling g.")
    g(0)
    fmt.Println("Returned normally from g.")
}

func g(i int) {
    if i > 3 {
        fmt.Println("Panicking")
        panic(fmt.Sprintf("%v", i))
    }
    defer fmt.Println("Defer in g", i)
    fmt.Println("Printing in g", i)
    g(i + 1)
}
```

## Error with info
* Implement error interface to add additional information on errors
```
func mainc() {
    _, err := sqrt(-10)
    if err != nil {
        log.Fatalln(err)
    }
}

func sqrt(f float64) (float64, error) {
    if f < 0 {
        return 0, errors.New("norgate math: square root of negative number")
    }
    return 42, nil
}
```
```
var ErrNorgateMath = errors.New("norgate math: square root of negative number")

func main() {
    fmt.Printf("%T\n", ErrNorgateMath)
    _, err := sqrt(-10)
    if err != nil {
        log.Fatalln(err)
    }
}

func sqrt(f float64) (float64, error) {
    if f < 0 {
        return 0, ErrNorgateMath
    }
    return 42, nil
}
```
```
func main() {
    fmt.Printf("%T\n", ErrNorgateMath)
    _, err := sqrt(-10)
    if err != nil {
        log.Fatalln(err)
    }
}

func sqrt(f float64) (float64, error) {
    if f < 0 {
        return 0, fmt.Errorf("norgate math again: square root of negative number: %v", f)
    }
    return 42, nil
}
```
