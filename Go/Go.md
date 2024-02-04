# Go Lang

## Notes
* Statically typed
* Go is mutable
* Go use references
```
a := []int{0, 1, 2, 3} // a = 0, 1, 2, 3
b := a // b = 0, 1, 2, 3

a[0] = 7

// a = 7, 1, 2, 3
// b = 7, 1, 2, 3
```
* Go pass by value for primitive and structures. And reference for slices and maps
```
func mySort(x []int) {
	sort.Ints(x)
}

func main() {
	o := []int{5, 2, 3, 4}
	mySort(o)

	fmt.Println(o) // 2, 3, 4, 5 => o is sorted
}

// Pass structure by value
test := MyStruct{
	value: 1,
}
hello(test)
fmt.Println(test.value) // Will be 1

type MyStruct struct {
	value int
}

func hello(v MyStruct) {
	v.value = 10
}

// Pass structure by reference
test := MyStruct{
	value: 1,
}
hello(&test)
fmt.Println(test.value) // Will be 10

func hello(v *MyStruct) {
	v.value = 10
}
```
* We can pass parameters as references
```
a := 1
test(&a)
fmt.Println(a)

func test(x *int) {
	*x++
}
```
* `defer` defers the execution of the statement until the surrounding function returns
```
func main() {
	defer fmt.Println("world")

	fmt.Println("hello")
}

// Prints
"hello"
"world"
```

## Variables
* Declare and initialize
```
var h int = 42 // Specify type

OR

a := 42 // Declare and Assign 

OR 

b, c, d, _, f := 0, 1, 2, 3, "happiness" // Multiple. _ is blank identitier

OR

var g in // Just declare
```
* Also const `const d int = 42`

## Functions
```
fun add(x, y int) int {
    return x + y
}

OR

fun add(x int, y int) int {
    return x + y
}
```
* `x` takes the type of `y` as it is not specified. It works for as many argument as necessary. The last one needs to have type at least
* Function can return multiple results
```
func swap(x, y string) (string, string) {
    return y, x
}

func main() {
    a, b := swap("hello", "world")
}
```
* Named return values are allows
```
func split(sum int) (x, y int) {
    x = sum * 4/9
    y = sum - x
    return
}
```

## Iota
* The iota keyword represents successive integer constants 0, 1, 2,…
* It resets to 0 whenever the word const appears in the source code,
* and increments after each const specification.
```
const (
	C0 = iota
	C1 = iota
	C2 = iota
)
fmt.Println(C0, C1, C2) // "0 1 2"

const (
    _           = iota // ignore first value by assigning to blank identifier
    KB ByteSize = 1 << (10 * iota)
    MB
    GB
    TB
    PB
    EB
    ZB
    YB
)
```

## Modules, packages, depedencies & visibility
* Modules have packages
* To create a module `go mod init <name>`
* Go module are new way of workspaces. A module is a 'project'
```
Module
  |
  --> Packages
```
* A module configures a workspace. It has a name, defines dependencies like `require golang.org/x/exp v...`
* `go get` is a tool to get a third party package to use in our code
* `go mod tidy` resolves automatically used dependencies and clean dependencies
* In Go, everything that is lower case is not exported
```
package mypackage

var isNotExported = 42 // not exported. private
```
* Something capitalized is exported and visible outside (from importers)
```
package mypackage

func Fascinating() { // exported. visible outside. public

}
```
* Use `import` to import packages from modules

## Commands
* `go build <file>` build the file
* `go build .` build the module
* `GOOS=windows go build` build for windows (can be linux, darwin [mac])
* `go install` install program in path
* `go help` to get help on commands or `go help <topic>`

## Init function
* An init function is run at the beginning of a program to setup state
```
func init() {
    // initialize
}

func main() {

}
```
