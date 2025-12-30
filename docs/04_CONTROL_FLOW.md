# Control Flow in Aether

## Conditional Execution

### The `if` Statement

Execute code based on a condition:

```aether
if condition {
    // code when true
}
```

### `if-else` Statement

```aether
if condition {
    // code when true
} else {
    // code when false
}
```

### Full Example

```aether
func classify_number(x: Int) -> Int {
    if x > 0 {
        return 1    // Positive
    } else {
        if x < 0 {
            return 0 - 1  // Negative
        } else {
            return 0    // Zero
        }
    }
}
```

### Nested Conditions

```aether
func grade_score(score: Int) -> Int {
    if score >= 90 {
        return 4    // A
    } else {
        if score >= 80 {
            return 3    // B
        } else {
            if score >= 70 {
                return 2    // C
            } else {
                if score >= 60 {
                    return 1    // D
                } else {
                    return 0    // F
                }
            }
        }
    }
}
```

### Condition Expression Details

The condition must evaluate to an integer:
- `0` = false (branch not taken)
- Non-zero = true (branch taken)

```aether
if 1 {
    // Always executes
}

if 0 {
    // Never executes
}

if x {
    // Executes if x != 0
}
```

### Multiple Conditions with &&

```aether
func in_range(x: Int, min: Int, max: Int) -> Int {
    if x >= min && x <= max {
        return 1
    }
    0
}
```

### Multiple Conditions with ||

```aether
func is_weekend(day: Int) -> Int {
    // 0=Sun, 6=Sat
    if day == 0 || day == 6 {
        return 1
    }
    0
}
```

## Loops

### The `while` Loop

```aether
while condition {
    // body
}
```

### Basic Loop Example

```aether
func count_to_10() -> Int {
    let i = 1
    while i <= 10 {
        // Do something with i
        i = i + 1
    }
    i   // Returns 11
}
```

### Sum Loop

```aether
func sum_1_to_n(n: Int) -> Int {
    let sum = 0
    let i = 1
    while i <= n {
        sum = sum + i
        i = i + 1
    }
    sum
}
```

### Early Exit with `return`

```aether
func find_first_even(start: Int, max: Int) -> Int {
    let i = start
    while i <= max {
        if i % 2 == 0 {
            return i    // Found! Exit loop and function
        }
        i = i + 1
    }
    0 - 1   // Not found
}
```

### Infinite Loop Pattern

```aether
func run_forever() -> Int {
    while 1 {
        // Loop body
        // Must have a return somewhere to exit
        if should_stop() {
            return 0
        }
    }
    0
}
```

### Nested Loops

```aether
func multiplication_sum(n: Int) -> Int {
    let total = 0
    let i = 1
    while i <= n {
        let j = 1
        while j <= n {
            total = total + (i * j)
            j = j + 1
        }
        i = i + 1
    }
    total
}
```

## Pattern: Simulate For Loop

Aether doesn't have `for`, but you can simulate it:

```aether
// for (int i = 0; i < n; i++)
func for_pattern(n: Int) -> Int {
    let i = 0
    while i < n {
        // Body here using i
        i = i + 1
    }
    0
}
```

## Pattern: Do-While

Simulate do-while (execute at least once):

```aether
func do_while_pattern() -> Int {
    let done = 0
    let first = 1
    
    while first || !done {
        first = 0
        // Body
        if condition {
            done = 1
        }
    }
    0
}
```

## Pattern: Break and Continue

Aether doesn't have `break`/`continue`, use flags:

```aether
// Simulate break
func with_break(n: Int) -> Int {
    let i = 0
    let found = 0
    while i < n && !found {
        if condition(i) {
            found = 1   // Acts like break
        } else {
            i = i + 1
        }
    }
    i
}

// Simulate continue
func with_continue(n: Int) -> Int {
    let i = 0
    let sum = 0
    while i < n {
        i = i + 1
        if should_skip(i) {
            // Just don't do the work - acts like continue
        } else {
            sum = sum + process(i)
        }
    }
    sum
}
```

## Control Flow Best Practices

### 1. Exit Early

```aether
// Good: Exit early
func validate(x: Int) -> Int {
    if x < 0 { return 0 }
    if x > 100 { return 0 }
    1
}

// Avoid: Deep nesting
func validate_nested(x: Int) -> Int {
    if x >= 0 {
        if x <= 100 {
            return 1
        }
    }
    0
}
```

### 2. Keep Loops Simple

```aether
// Good: Clear loop with single purpose
func sum_array(arr: Int, len: Int) -> Int {
    let sum = 0
    let i = 0
    while i < len {
        sum = sum + ae_load64(arr + i * 8)
        i = i + 1
    }
    sum
}
```

### 3. Avoid Deep Nesting

Maximum recommended nesting: 3 levels. If deeper, extract to functions.
