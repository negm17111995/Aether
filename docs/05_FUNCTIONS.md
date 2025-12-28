# PART 5: FUNCTIONS

## 5.1 Defining Functions

### 5.1.1 Basic Syntax

```aether
func function_name(param1: Type1, param2: Type2) -> ReturnType {
    // function body
    return_value
}
```

### 5.1.2 Simple Function

```aether
func greet() {
    print("Hello!")
}

// Call it:
greet()  // Output: Hello!
```

### 5.1.3 Function with Parameters

```aether
func greet_person(name: String) {
    print(f"Hello, {name}!")
}

greet_person("Alice")  // Hello, Alice!
greet_person("Bob")    // Hello, Bob!
```

### 5.1.4 Function with Return Value

```aether
func add(a: Int, b: Int) -> Int {
    a + b  // Last expression is returned
}

let sum = add(3, 5)  // 8
```

### 5.1.5 Multiple Parameters

```aether
func calculate(x: Int, y: Int, z: Int) -> Int {
    x * y + z
}

let result = calculate(2, 3, 4)  // 2*3+4 = 10
```

---

## 5.2 Return Values

### 5.2.1 Implicit Return

The last expression is the return value:

```aether
func square(x: Int) -> Int {
    x * x  // Returned automatically
}
```

### 5.2.2 Explicit Return

Use `return` keyword for early exit:

```aether
func absolute(x: Int) -> Int {
    if x < 0 {
        return -x  // Early return
    }
    x  // Normal return
}
```

### 5.2.3 No Return Value

Functions without `->` return nothing:

```aether
func log_message(msg: String) {
    print(f"[LOG] {msg}")
    // No return value
}
```

### 5.2.4 Returning Multiple Values

Use tuples:

```aether
func divide_with_remainder(a: Int, b: Int) -> (Int, Int) {
    (a / b, a % b)
}

let (quotient, remainder) = divide_with_remainder(17, 5)
// quotient = 3, remainder = 2
```

---

## 5.3 Parameters

### 5.3.1 Pass by Value (Default)

Parameters are copied by default:

```aether
func try_modify(x: Int) {
    // x = 10  // ERROR: x is immutable
    print(x)
}

let value = 5
try_modify(value)
// value is still 5
```

### 5.3.2 Mutable Parameters

Use `mut` for mutable local copy:

```aether
func increment(mut x: Int) -> Int {
    x = x + 1  // Modifies local copy
    x
}

let value = 5
let result = increment(value)
// value = 5 (unchanged)
// result = 6
```

### 5.3.3 Reference Parameters

Use `&` to pass by reference:

```aether
func calculate_length(s: &String) -> Int {
    s.len()  // Read the string without copying
}

let text = "Hello, World!"
let len = calculate_length(&text)  // 13
```

### 5.3.4 Mutable Reference Parameters

Use `&mut` to modify in place:

```aether
func double_in_place(x: &mut Int) {
    *x = *x * 2
}

let mut value = 5
double_in_place(&mut value)
// value = 10
```

---

## 5.4 Closures (Anonymous Functions)

### 5.4.1 Basic Closure Syntax

```aether
let add_one = |x: Int| -> Int {
    x + 1
}

let result = add_one(5)  // 6
```

### 5.4.2 Type Inference in Closures

```aether
// Types can be inferred
let double = |x| x * 2

let result = double(5)  // 10
```

### 5.4.3 Closures Capture Environment

```aether
let multiplier = 3

let multiply = |x| x * multiplier  // Captures multiplier

let result = multiply(5)  // 15
```

### 5.4.4 Move Closures

Transfer ownership into closure:

```aether
let text = "Hello"

let printer = move || {
    print(text)  // text is moved into closure
}

// text is no longer valid here
printer()  // Hello
```

### 5.4.5 Closures as Parameters

```aether
func apply(x: Int, f: fn(Int) -> Int) -> Int {
    f(x)
}

let result = apply(5, |x| x * 2)  // 10

// Or with named function
func triple(x: Int) -> Int { x * 3 }
let result2 = apply(5, triple)  // 15
```

---

## 5.5 Higher-Order Functions

### 5.5.1 Functions That Take Functions

```aether
func map_array(arr: [Int], f: fn(Int) -> Int) -> [Int] {
    let mut result = []
    for item in arr {
        result.push(f(item))
    }
    result
}

let numbers = [1, 2, 3, 4, 5]
let doubled = map_array(numbers, |x| x * 2)
// [2, 4, 6, 8, 10]
```

### 5.5.2 Functions That Return Functions

```aether
func make_adder(n: Int) -> fn(Int) -> Int {
    |x| x + n  // Returns a closure
}

let add_5 = make_adder(5)
let add_10 = make_adder(10)

let a = add_5(3)   // 8
let b = add_10(3)  // 13
```

### 5.5.3 Common Higher-Order Functions

```aether
let numbers = [1, 2, 3, 4, 5]

// Map: transform each element
let doubled = numbers.map(|x| x * 2)
// [2, 4, 6, 8, 10]

// Filter: keep elements matching predicate
let evens = numbers.filter(|x| x % 2 == 0)
// [2, 4]

// Reduce: combine all elements
let sum = numbers.reduce(0, |acc, x| acc + x)
// 15

// Any: check if any match
let has_even = numbers.any(|x| x % 2 == 0)
// true

// All: check if all match
let all_positive = numbers.all(|x| x > 0)
// true

// Find: get first match
let first_even = numbers.find(|x| x % 2 == 0)
// Some(2)
```

---

## 5.6 Recursion

### 5.6.1 Basic Recursion

```aether
func factorial(n: Int) -> Int {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

let f5 = factorial(5)  // 120
```

### 5.6.2 Tail Recursion

Aether optimizes tail-recursive functions:

```aether
func factorial_tail(n: Int, acc: Int) -> Int {
    if n <= 1 {
        acc
    } else {
        factorial_tail(n - 1, n * acc)  // Tail call
    }
}

func factorial(n: Int) -> Int {
    factorial_tail(n, 1)
}

let f100 = factorial(100)  // Works! (TCO prevents stack overflow)
```

### 5.6.3 Mutual Recursion

```aether
func is_even(n: Int) -> Bool {
    if n == 0 { true }
    else { is_odd(n - 1) }
}

func is_odd(n: Int) -> Bool {
    if n == 0 { false }
    else { is_even(n - 1) }
}

let a = is_even(10)  // true
let b = is_odd(10)   // false
```

---

## 5.7 Generic Functions

### 5.7.1 Type Parameters

```aether
func identity<T>(x: T) -> T {
    x
}

let a = identity(5)        // Int
let b = identity("hello")  // String
```

### 5.7.2 Multiple Type Parameters

```aether
func pair<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}

let p = pair(42, "hello")  // (Int, String)
```

### 5.7.3 Trait Bounds

```aether
func print_twice<T: Printable>(x: T) {
    print(x.to_string())
    print(x.to_string())
}

func max<T: Comparable>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

### 5.7.4 Where Clauses

```aether
func process<A, B>(a: A, b: B) -> Int
where
    A: Printable + Clone,
    B: Comparable
{
    // ...
}
```

---

## 5.8 Function Attributes

### 5.8.1 Inline

```aether
#[inline]
func fast_add(a: Int, b: Int) -> Int {
    a + b
}

#[inline(always)]
func critical_path(x: Int) -> Int {
    x * x
}

#[inline(never)]
func debug_only(msg: String) {
    print(f"DEBUG: {msg}")
}
```

### 5.8.2 Test Functions

```aether
#[test]
func test_addition() {
    assert(add(2, 3) == 5)
}

#[test]
#[should_panic]
func test_divide_by_zero() {
    divide(1, 0)  // Should panic
}
```

### 5.8.3 Deprecated

```aether
#[deprecated("Use new_function instead")]
func old_function() {
    // ...
}
```

---

*Next: [Part 6: Data Structures](06_DATA_STRUCTURES.md)*
