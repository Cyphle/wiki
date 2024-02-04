# Go Structures

## Array
* Size does not change
* Most of the time slices are reommanded
```
var x [5]int
x[3] = 42

// Array literal
y :=[4]int{55, 56, 57, 58}

// Array literal, compiler count nb of elements
z := [...]string{"chocolate", "vanilla", "strawberry"}
```

## Slice
* Built on top or an array
* Length is the number of contained elements `len()`
* Capacity is the size of the slice's underlying array `cap()`
* Changes size
```
xs := []string{"Almond", "Banan", "Balsamic"}

for index, value := range xs {
    ...
}

xs = append(xs, "Coucou")
```
* A slice of a slice
```
xi := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}

xi[1:3]

xi[:3]
xi[1:]
xi[:] // == xi
```
* Deleting from a slice
```
xi := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}

xi = append(xi[:4], xi[5:]...) // Copy without 4th element
```
* `...` is the spread operator to make slice into multiple variables `<slice>...` (comes after variable)
* `make` function
```
xi := make([]int, 5) // Create array of 5 0 : {0, 0, 0, 0, 0}
xi := make([]int, 0, 10) // Create a slice of 0 elements and capacity of 10
```

## Map
* Unordered
```
myMap := map[string]int {
    "Todd": 42,
    "Henry": 16
}
```
* With `make`
```
am := make(map[string]int)
am["Lucas"] = 28
```
* Iterating
```
for key, value := range myMap {

}

for _, value := range myMap {
    // To ignore key
}
```
* Deleting element
```
delete(myMap, <myKey>)
```
* Comma ok idiom
```
v, ok := myMap["key"]
if ok {

} else {
    fmt.Println("Key doesn't exist")
}

if v, ok := an["Lucas"]; !ok {
    fmt.Println("Key doesn't exist")
} else {
    fmt.Println("value is", v)
}
```
* Accessing a non existing element does not make the program panic but return a default value like 0 for int maps or empty string for string maps

## Struct
* Composite type
```
type person struct {
    first string
    last string
    age int
}

p1 := person{
    first: "John",
    last: "Smith",
    age: 32,
}

type secretAgent struct {
    person
    licenceToKill bool
}

sa := secretAgent {
    person: person {
        first: "John",
        last: "Smith",
        age: 32,
    },
    licenceToKill: true,
}
```
* Anonymous struct
```
myStruct := struct {
    first string
    last string
    age int
} {
    first: "John",
    last: "Smith",
    age: 32
}
```
* Compositon: when a struct embed another struct, the parent includes all fields and methods of child
```
func main() {
	car := Car{
		Engine: Engine{
			mode: "Essence",
		},
	}

	car.start()
}

type Engine struct {
	mode string
}

func (e *Engine) start() {
	fmt.Println("Engine started")
}

type Car struct {
	Engine
}

```
* Go is object oriented. It has polymorphism (interface), overriding (promotion), inheritence, encapsulation

## Interfaces
```
type Forme interface { // création de L'interface Forme
    Air() float64 // signature de la méthode Air()
    Perimetre() float64 // signature de la méthode Perimetre()
}

type Rectangle struct {
    largeur  float64
    longueur float64
}

/* 
Pour implémenter une interface dans Go, il suffit
d'implémente toutes les méthodes de l'interface. Ici on
implémente la méthode Air() de l'interface Forme.
 */
func (r Rectangle) Air() float64 {
	return r.largeur * r.longueur
}

/*
On implémente la méthode Perimetre() de l'interface Forme
*/
func (r Rectangle) Perimetre() float64 {
	return 2 * (r.largeur * r.longueur)
}

func main() {
    var f Forme
    f = Rectangle{5.0, 4.0} // affectation de la structure Rectangle à l'interface Forme 
    r := Rectangle{5.0, 4.0} 
    fmt.Println("Type de f :", f)
    fmt.Printf("Valeur de f : %v\n", f)
    fmt.Println("Air du rectangle r :", f.Air())
    fmt.Println("f == r ? ", f == r)
}
```