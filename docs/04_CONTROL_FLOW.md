# PART 4: CONTROL FLOW

## 4.1 If Expressions

### 4.1.1 Basic If

```aether
let x = 10

if x > 5 {
    print("x is greater than 5")
}
```

**Key points:**
- Condition must be a `Bool` (no truthy/falsy)
- Braces `{}` are required
- No parentheses needed around condition

### 4.1.2 If-Else

```aether
let x = 3

if x > 5 {
    print("greater")
} else {
    print("not greater")
}
```

### 4.1.3 Else-If Chain

```aether
let score = 85

if score >= 90 {
    print("A")
} else if score >= 80 {
    print("B")
} else if score >= 70 {
    print("C")
} else if score >= 60 {
    print("D")
} else {
    print("F")
}
```

### 4.1.4 If as an Expression

In Aether, `if` returns a value:

```aether
let x = 10
let sign = if x > 0 { "positive" } else { "non-positive" }
// sign = "positive"

let abs_value = if x < 0 { -x } else { x }
// abs_value = 10
```

**Rule:** Both branches must return the same type:

```aether
// ERROR: branches have different types
let bad = if true { 5 } else { "hello" }

// OK: both Int
let good = if true { 5 } else { 10 }
```

### 4.1.5 Nested If

```aether
let x = 10
let y = 20

if x > 5 {
    if y > 15 {
        print("both conditions true")
    } else {
        print("x > 5, but y <= 15")
    }
} else {
    print("x <= 5")
}
```

---

## 4.2 Match Expressions

### 4.2.1 Basic Match

`match` is pattern matching (like switch, but much more powerful):

```aether
let x = 2

match x {
    1 => print("one"),
    2 => print("two"),
    3 => print("three"),
    _ => print("something else"),
}
```

**Key points:**
- `_` is the wildcard pattern (matches anything)
- Match must be exhaustive (cover all possibilities)
- Each arm ends with `,`

### 4.2.2 Match as Expression

```aether
let x = 2

let name = match x {
    1 => "one",
    2 => "two",
    3 => "three",
    _ => "unknown",
}
// name = "two"
```

### 4.2.3 Multiple Patterns

Use `|` to match multiple values:

```aether
let x = 5

match x {
    1 | 2 | 3 => print("1, 2, or 3"),
    4 | 5 | 6 => print("4, 5, or 6"),
    _ => print("other"),
}
```

### 4.2.4 Range Patterns

```aether
let age = 25

match age {
    0..=12 => print("child"),
    13..=19 => print("teenager"),
    20..=64 => print("adult"),
    65..=120 => print("senior"),
    _ => print("invalid age"),
}
```

**Range syntax:**
- `0..10` — 0 to 9 (exclusive end)
- `0..=10` — 0 to 10 (inclusive end)

### 4.2.5 Guards

Add conditions with `if`:

```aether
let x = 5

match x {
    n if n < 0 => print("negative"),
    n if n == 0 => print("zero"),
    n if n % 2 == 0 => print("positive even"),
    n => print("positive odd"),
}
```

### 4.2.6 Destructuring in Match

**Tuples:**
```aether
let point = (3, 4)

match point {
    (0, 0) => print("origin"),
    (0, y) => print(f"on y-axis at {y}"),
    (x, 0) => print(f"on x-axis at {x}"),
    (x, y) => print(f"at ({x}, {y})"),
}
```

**Structs:**
```aether
struct Point { x: Int, y: Int }

let p = Point { x: 3, y: 4 }

match p {
    Point { x: 0, y: 0 } => print("origin"),
    Point { x, y: 0 } => print(f"on x-axis at {x}"),
    Point { x: 0, y } => print(f"on y-axis at {y}"),
    Point { x, y } => print(f"at ({x}, {y})"),
}
```

**Enums:**
```aether
enum Message {
    Quit,
    Move { x: Int, y: Int },
    Write(String),
}

let msg = Message::Move { x: 10, y: 20 }

match msg {
    Message::Quit => print("quit"),
    Message::Move { x, y } => print(f"move to {x}, {y}"),
    Message::Write(text) => print(f"write: {text}"),
}
```

---

## 4.3 While Loops

### 4.3.1 Basic While

```aether
let mut i = 0

while i < 5 {
    print(i)
    i = i + 1
}
// Output: 0 1 2 3 4
```

### 4.3.2 While with Complex Condition

```aether
let mut x = 100

while x > 1 && x % 2 == 0 {
    x = x / 2
    print(x)
}
// Output: 50 25
```

### 4.3.3 Infinite Loop with While

```aether
while true {
    // Runs forever
    // Use break to exit
    if should_stop() {
        break
    }
}
```

---

## 4.4 For Loops

### 4.4.1 For-In with Ranges

```aether
// Exclusive range (0 to 4)
for i in 0..5 {
    print(i)
}
// Output: 0 1 2 3 4

// Inclusive range (0 to 5)
for i in 0..=5 {
    print(i)
}
// Output: 0 1 2 3 4 5
```

### 4.4.2 For-In with Arrays

```aether
let fruits = ["apple", "banana", "cherry"]

for fruit in fruits {
    print(fruit)
}
// Output: apple banana cherry
```

### 4.4.3 For with Index

```aether
let items = ["a", "b", "c"]

for (index, item) in items.enumerate() {
    print(f"{index}: {item}")
}
// Output:
// 0: a
// 1: b
// 2: c
```

### 4.4.4 For with Step

```aether
// Count by 2
for i in (0..10).step_by(2) {
    print(i)
}
// Output: 0 2 4 6 8

// Reverse
for i in (0..5).rev() {
    print(i)
}
// Output: 4 3 2 1 0
```

---

## 4.5 Loop (Infinite)

### 4.5.1 Basic Loop

`loop` creates an infinite loop:

```aether
loop {
    // Runs forever until break
    let input = read_line()
    if input == "quit" {
        break
    }
    process(input)
}
```

### 4.5.2 Loop with Return Value

`loop` can return a value via `break`:

```aether
let mut counter = 0

let result = loop {
    counter = counter + 1
    if counter == 10 {
        break counter * 2
    }
}
// result = 20
```

---

## 4.6 Break and Continue

### 4.6.1 Break

Exit a loop immediately:

```aether
for i in 0..100 {
    if i == 5 {
        break  // Exit loop
    }
    print(i)
}
// Output: 0 1 2 3 4
```

### 4.6.2 Continue

Skip to next iteration:

```aether
for i in 0..10 {
    if i % 2 == 0 {
        continue  // Skip even numbers
    }
    print(i)
}
// Output: 1 3 5 7 9
```

### 4.6.3 Labeled Loops

Break out of nested loops:

```aether
'outer: for i in 0..10 {
    for j in 0..10 {
        if i * j > 20 {
            break 'outer  // Exit both loops
        }
        print(f"{i} * {j} = {i * j}")
    }
}
```

Continue outer loop:

```aether
'outer: for i in 0..5 {
    for j in 0..5 {
        if j == 2 {
            continue 'outer  // Skip to next i
        }
        print(f"({i}, {j})")
    }
}
```

---

## 4.7 Early Return

### 4.7.1 Return Statement

Exit a function early:

```aether
func find(arr: [Int], target: Int) -> Int {
    for i in 0..arr.len() {
        if arr[i] == target {
            return i  // Found! Exit immediately
        }
    }
    -1  // Not found
}
```

### 4.7.2 Implicit Return

The last expression is returned:

```aether
func double(x: Int) -> Int {
    x * 2  // No return keyword needed
}

func max(a: Int, b: Int) -> Int {
    if a > b { a } else { b }  // Expression returned
}
```

---

*Next: [Part 5: Functions](05_FUNCTIONS.md)*
