# Functions in Aether

## Function Basics

### Syntax

```aether
func function_name(param1: Type1, param2: Type2) -> ReturnType {
    // body
    expression  // Last expression is return value
}
```

### Minimal Function

```aether
func get_answer() -> Int {
    42
}
```

### Function with Parameters

```aether
func add(a: Int, b: Int) -> Int {
    a + b
}
```

### Calling Functions

```aether
func main() -> Int {
    let x = get_answer()    // 42
    let y = add(10, 20)     // 30
    x + y                   // 72
}
```

## Parameters

### Parameter Passing

All parameters are passed by value (copy):

```aether
func double(x: Int) -> Int {
    x * 2   // x is a copy
}

func main() -> Int {
    let a = 21
    let b = double(a)   // a is copied, not modified
    // a is still 21, b is 42
    a + b
}
```

### Multiple Parameters

Up to 8 parameters use registers (fast):

```aether
func compute(a: Int, b: Int, c: Int, d: Int) -> Int {
    a + b + c + d
}
```

### Parameter Naming Conventions

```aether
// Use descriptive names
func calculate_area(width: Int, height: Int) -> Int {
    width * height
}

// Not: func f(w: Int, h: Int) -> Int
```

## Return Values

### Implicit Return

The last expression is returned:

```aether
func square(x: Int) -> Int {
    x * x   // Returned implicitly
}
```

### Explicit Return

Use `return` for early exit:

```aether
func absolute(x: Int) -> Int {
    if x < 0 {
        return 0 - x
    }
    x
}
```

### Multiple Return Points

```aether
func classify(x: Int) -> Int {
    if x > 100 {
        return 3    // High
    }
    if x > 50 {
        return 2    // Medium
    }
    if x > 0 {
        return 1    // Low
    }
    0               // Zero or negative
}
```

## Recursion

### Basic Recursion

```aether
func factorial(n: Int) -> Int {
    if n <= 1 {
        return 1
    }
    n * factorial(n - 1)
}
```

### Fibonacci

```aether
func fib(n: Int) -> Int {
    if n <= 1 {
        return n
    }
    fib(n - 1) + fib(n - 2)
}
```

### Tail Recursion (Optimizable)

```aether
func factorial_tail(n: Int, acc: Int) -> Int {
    if n <= 1 {
        return acc
    }
    factorial_tail(n - 1, n * acc)
}

func factorial(n: Int) -> Int {
    factorial_tail(n, 1)
}
```

## Function Patterns

### Predicates (Boolean-returning)

```aether
func is_even(x: Int) -> Int {
    x % 2 == 0
}

func is_positive(x: Int) -> Int {
    x > 0
}

func is_valid_age(age: Int) -> Int {
    age >= 0 && age <= 150
}
```

### Transformers

```aether
func double(x: Int) -> Int { x * 2 }
func triple(x: Int) -> Int { x * 3 }
func negate(x: Int) -> Int { 0 - x }
func increment(x: Int) -> Int { x + 1 }
```

### Accumulators

```aether
func sum_range(start: Int, end: Int) -> Int {
    let sum = 0
    let i = start
    while i <= end {
        sum = sum + i
        i = i + 1
    }
    sum
}
```

### Finders

```aether
func find_max(arr: Int, len: Int) -> Int {
    let max = ae_load64(arr)
    let i = 1
    while i < len {
        let val = ae_load64(arr + i * 8)
        if val > max {
            max = val
        }
        i = i + 1
    }
    max
}
```

## The main() Function

### Entry Point

Every Aether program needs `main`:

```aether
func main() -> Int {
    0   // Exit code 0 = success
}
```

### Exit Codes

```aether
func main() -> Int {
    if operation_failed() {
        return 1    // Error
    }
    0               // Success
}
```

### Common Exit Code Convention

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid usage |
| 126 | Permission denied |
| 127 | Command not found |
| 128+ | Signal-related |

## Function Best Practices

### 1. Single Responsibility

```aether
// Good: One purpose
func validate_input(x: Int) -> Int { x > 0 && x < 100 }
func process_input(x: Int) -> Int { x * 2 }
func output_result(x: Int) -> Int { x }

// Bad: Multiple purposes
func do_everything(x: Int) -> Int {
    if x <= 0 || x >= 100 { return 0 - 1 }
    let processed = x * 2
    processed
}
```

### 2. Meaningful Names

```aether
// Good
func calculate_compound_interest(principal: Int, rate: Int, years: Int) -> Int

// Bad
func calc(p: Int, r: Int, y: Int) -> Int
```

### 3. Limit Parameters

```aether
// Good: Few params
func create_point(x: Int, y: Int) -> Int

// Consider: If many params, use a struct pattern
func create_rect(x: Int, y: Int, w: Int, h: Int) -> Int
```

### 4. Document Complex Functions

```aether
// Calculate the greatest common divisor using Euclidean algorithm
// Time: O(log(min(a,b)))
// Returns: GCD of a and b
func gcd(a: Int, b: Int) -> Int {
    while b != 0 {
        let t = b
        b = a % b
        a = t
    }
    a
}
```
