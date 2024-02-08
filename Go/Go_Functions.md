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
```

```