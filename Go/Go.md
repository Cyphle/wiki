# Go Lang

## Notes
* Statically typed
* Go is mutable

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

## Modules & depedencies
* Modules have packages
* To create a module `go mod init <name>`