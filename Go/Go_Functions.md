# Go Functions

## Syntax
```
func myFunction(x int) int {

}

func hello(name string, age int) (string, int) {
    return "Hello", 12
}
```
* Parameters are passed by value by default (except for slices and maps)

## Variatic parameters
```
func sum(x ...int) int {
    n := 0
    for _, v := range x {
        n += v
    }
    return n
}

xi := []int{1, 2, 3, 4}
x := sum(xi...)
```

## Defer
* `defer` allows to execute a function at the end of a function
```
func main() {
    defer foo() // Will be executed after bar, at the end of main
    bar()
}

// Elegant way to say to close a file
f, err := os.open("file.txt")
if err != nil {
    log.Fatalf("error: %s", err)
}
defer f.Close() // It is declared closed to opening but will be executed at the end
```

## Methods
* Functions of struct
```
type person struct {
    first string
}

func (p person) speak() {
    ...
}

func main() {
    p1 := person{
        first: "James",
    }

    p1.speak()
}
```

## Interfaces & polymorphism
```
type person struct {
    first string
}

type secretAgent struct {
    person
    ltk bool
}

func (p person) speak() {
    fmt.Println("Hello")
}

func (sa secretAgent) speak() {
    fmt.Println("I am a secret agent")
}

type human interface {
    speak()
}

func saySomething(h human) {
    h.speak()
}

func main() {
    sa1 := secretAgent {
        person: person {
            first: "James"
        },
        ltk: true,
    }

    p2 := person {
        first: "Jenny"
    }

    // sa1.speak()
    // p2.speak()

    saySomething(sa1)
    saySomething(p2)
}
```
* Interface allows to use abstract types for functions like `saySomething`
* Implement interface Stringer to have struct having same methods as strings (it is like the toString())

## Writer interface
* Any type which implements `Writer` interface can be pass to mathod to write in file

## Anonymous function
```
func main() {
    // This is an anonymous function
    func() {
        // Do something
    }
}
```

## Functions as variable
* Function as variable
```
func main() {
    x := func() {
        // Do something
    }

    x()
}
```
* Returning a function
```
func bar() func() int {
    return func() int {
        return 42
    }
}

func main() {
    y := bar()
    y()
}
```
* Function as parameter/callback
```
func doMath(a int, b int, f func(int, int) int) int {
    return f(a, b)
}

func add(a int, b int) int {
    return a + b
}

func subtract(a int, b int) int {
    return a - b
}

func main() {
    x := doMath(42, 16, add)
}
```

## Closure
* A closure is a function that captures a state
```
func incrementor() func() int {
    x := 0
    return func() int {
        x++
        return x
    }
}

func main() {
    f := incrementor() // x is inclosed in f so f captures that state of x and can increment it
    fmt.Println(f()) // 1
	fmt.Println(f()) // 2
	fmt.Println(f()) // 3
	fmt.Println(f()) // 4
	fmt.Println(f()) // 5

    g ;= incrementor()
    fmt.Println(g()) // 1
    fmt.Println(g()) // 2
    fmt.Println(g()) // 3
    fmt.Println(g()) // 4
}
```

## Recursion
```
func factorial(n int) int {
    if n == 0 {
        return 1
    }
    return n * factorial(n - 1)
}
```

## Wrapper function
* A wrapper function is a function that provides an additional layer of abstraction or functionality around an existing function. Used for example for logging or profiling
```
func TimeFunction(fn func()) {
    start := time.Now()
    fn()
    // Do something else
}
```