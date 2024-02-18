# Go pointers

## Go principles
* Go pass by value (meaning it copies things) for primitive and structures. And reference for slices and maps
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
* As go pass by value, to mutate, use pointers
```
func intDelta(n *int) {
    *n = 43
}

func intDeltaByValue(n int) {
    n = 44
}

func main() {
    a := 42
    fmt.Println(a) // 42
    intDelta(&a)
    fmt.Println(a) // 43
    intDeltaByValue(a)
    fmt.Println(a) // Still 43
}
```
* <b>Slices and maps</b> are passed by reference ! They are the exceptions
```
func sliceDelta(ii []int) {
    ii[0] = 99
}

func mapDelta(md map[string]int, s string) {
    md[s] = 33
}

func main() {
    m := []int{1, 2, 3, 4}

    fmt.Println(m) // 1, 2, 3, 4
    sliceDelta(m)
    fmt.Println(m) // 99, 2, 3, 4

    m := make(map[string]int)
    m["James"] = 32
    fmt.Println(m["James"]) // 32
    mapDelta(m, "James")
    fmt.Println(m["James"]) // 33
}
```

## Dereferencing
```
x := 42

y := &x // A pointer to x as y stores address of x

fmt.Println(*y) // Dereferencing y to print value of x
```

## Stack & Heap
* Value semantic is when passing by value
* Pointer semantic is when passing by reference/pointer
* Value semantic put variables on the stack
* Pointers point to variable on the heap
* Using pointers is more when sharing and mutation rather than memory optimization

## Method sets
* The method set of a type T consists of all methods with receiver type T
* The method set of a type *T consists of all methods with receiver *T or T
* Method set is integral to how interfaces are implemented
```
type dog struct {
    first string
}

func (d dog) walk() {
    fmt.Println("Walk walk")
}

func (d *dog) run() {
    d.first = "Rover"
    fmt.Println("Run run")
}

type youngin interface {
    walk()
    run()
}

func youngRun(y youngin) {
    y.run()
}

func main() {
    d1 := dog("Henry")
    d1.walk()
    d1.run()
    // youngRun(d1) => does not work because d1 is a value and is not a receiver for run and so not recognized as implementing youngin

    d2 := &dog("Padget")
    d2.walk()
    d2.run()
    youngRun(d2)
}
```