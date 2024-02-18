# Go Generics

```
func addT[T int | float64](a, b T) T {
    return a + b
}

func main() {
    addT(1, 2)
    addT(1.2, 2.2)
}
```
* Type constraint
```
type myNumbers interface {
    int | float
}

func addT[T myNumbers](a, b T) T {
    return a + b
}
```
* Type alias
```
type myAlias int
```
* Package constraint: Package constraints defines a set of useful constraints to be used with type parameters.
