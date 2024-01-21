# Go Control Flow

## If else
```
if x < 42 {
    // do something
} else if x == 42 {
    // do something else
} else {
    // another way
}
```
* Statement statement idiom: complex if statement
```
// Scope of z is contained in if else
if x := 2 * rand.Intn(x); z >= x {
    println(z)
} else {
    println(z)
}
```
* Ok idiom: almost same but with boolean
```
if seconds, ok := timeZone[tz]; ok {
    return seconds
}
``` 

## Switch
```
switch {
    case x < 42:
        println
    case x == 42:
        ...
    default:
        ...
}
```
or
```
switch x {
    case 42:
        ...
    case 41:
        ...
}
```
* `fallthrough`: by default, when one arm match, the `switch` returns. With `fallthrough` it runs the next arm
```
switch x {
    case 40:
        ...
        fallthrough
    case 42:
        ...
    case 41:
        ...
} // This will run case 40 AND 42
```

## Select
```
// Channels
ch1 := make(chan int)
ch2 := make(chan int)

d1 := time.Duration(rand.Int63n(250))
d2 := time.Duration(rand.Int63n(250))

// Coroutine
go func() {
    time.Sleep(d1 * time.Millisecond)
    ch1 <- 41
}

// Coroutine
go func() {
    time.Sleep(d2 * time.Millisecond)
    ch2 <- 42
}

select {
    case v1 := <-ch1:
        Println(v1)
    case v2 := <-ch2:
        Println(v2)
}
```
* Select will check which channel has an incoming value then runs the arm/branch

## For loop
```
for i := 0; i < 5; i++ {
    ...
}

for y < 10 {
    y++
}
```
* Using `break` to stop a loop
```
for {
    if y > 20 {
        break
    }
    y++
}
```
* Using `continue` to go right to next iteration
```
for i := 0; i < 20; i++ {
    if i%2 != 0 {
        continue
    }
}
```

## For range
* For range a list
```
xi := int[]{42, 43, 44, 45, 46, 47}
for i, v := range xi {
    Println(i, " is index", v, " is value")
}
```
* For range a map
```
m := map[string]int {
    "James": 42,
    "Moneypenny": 32
}
for k, v := range m {
    Println(k, " is key", v, " is value")
}

for _, v := range m {
    // To skip keys
}
```