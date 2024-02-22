# Go testing

* Test files are `*_test.go`
* Test functions are `func TestXxx(*testing.T)`
* Run tests with `go test`
```
func main() {
    fmt.Println(mySum(2, 3))
}

func mySum(xi ...int) int {
    sum := 0
    for i, y := range xi {
        sum += v
    }
    return sum
}
```
```
// main_test.go
func TestMySum(t *testing.T) {
    if mySum(2, 3) != 5 {
        t.Error("Expected", 5, "Got", x)
    }
}
```

## Table tests
```
func TestMySum(t *testing.T) {
    type test struct {
        data []int
        answer int
    }

    tests := []test{
        test{[]int{21, 21}, 42}
        test{[]int{3, 4, 5}, 12}
    }

    for _, v := range tests {
        x := mySum(v.data...)
        if x != v.answer {
            t.Error("Expected", v.answer, "Got", x)
        }
    }
}
```

## Test example
* They are tests that can be used as example documentation
```
func main() {
    acdc.MySum(2, 3)
}

// file acdc
package acdc

func MySum(xi ...int) int {
    sum := 0
    for i, y := range xi {
        sum += v
    }
    return sum
}

// main_test.go
func ExampleSum() {
    fmt.Println(MySum(2, 3))
    // Output
    // 5
}
```
* Use `godoc -http=:8080` and go to `localhost:8080`

## Golint
* Go lint to lint
* `go vet ./..` to lint all code 

## Benchmarking
* Allow to measure the speed of a program
```
func Greet(s string) string {
    return fmt.Sprintf("Welcome my dear ", s)
}

func main() {
    fmt.Println(saying.Great("James"))
}

// main_test.go
func TestGreet(t *testing.T) {
    s := Greet("James")
    if s != "Welcome my dear James" {
        t.Error("got", s, "expected", "Welcome my dear James")
    }
}

func ExampleGreet() {
    fmt.Println(Greet("James"))
    // Output:
    // Welcome my dead James
}

func BenchmarkGreet(b *testing.B) {
    for i := 0; i < b.N; i++ {
        Greet("James")
    }
}
```
* Run `go test -bench .` to run benchmark
* Or run `go test -bench <name>` like Greet in the example above

## Coverage
* Run `go run test -coverprofile c.out`
* To show result in browser `go tool cover -html=c.out`