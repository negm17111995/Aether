# Types and Variables in Aether

## The Type System

Aether uses a **static, strongly-typed** type system. Every expression has a known type at compile time.

## Primitive Types

### Int (64-bit Signed Integer)

The primary numeric type in Aether is `Int`, a 64-bit signed integer.

**Range**: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

```aether
let small: Int = 42
let large: Int = 9223372036854775807
let negative: Int = 0 - 1000
```

**Operations on Int:**
```aether
// Arithmetic
let sum = a + b
let diff = a - b
let product = a * b
let quotient = a / b    // Integer division (truncates)
let remainder = a % b

// Comparison (returns Bool-like Int: 0 or 1)
let is_equal = a == b
let is_greater = a > b

// Bitwise (via manual computation)
let shifted = x * 2     // Left shift by 1
let halved = x / 2      // Right shift by 1
```

### Bool (Boolean)

Boolean values are represented as integers internally:
- `true` = 1
- `false` = 0

```aether
let flag: Bool = true
let done = false

// Logical operations
let both = a && b       // AND
let either = a || b     // OR
let negated = !a        // NOT
```

## Type Annotations

### Explicit Type Declaration

```aether
let x: Int = 42
let flag: Bool = true
```

### Type Inference

When the type is obvious, you can omit the annotation:

```aether
let x = 42          // Inferred as Int
let y = x + 10      // Inferred as Int
let flag = true     // Inferred as Bool
let result = a > b  // Inferred as Int (0 or 1)
```

## Variables

### Declaration with `let`

```aether
let variable_name = initial_value
let variable_name: Type = initial_value
```

### Mutability

All variables declared with `let` are mutable:

```aether
let counter = 0
counter = counter + 1   // OK: reassignment allowed
counter = 100           // OK: can change value
```

### Variable Scope

Variables are scoped to their enclosing block:

```aether
func example() -> Int {
    let outer = 10
    
    if true {
        let inner = 20      // Only visible in this block
        outer = outer + inner
    }
    
    // inner is not accessible here
    outer   // Returns 30
}
```

### Shadowing

You can redeclare a variable with the same name:

```aether
let x = 10
let x = x * 2       // New x, value = 20
let x = x + 5       // New x, value = 25
```

## Constants

### Declaration with `const`

Constants must be compile-time known values:

```aether
const MAX_SIZE: Int = 1024
const BUFFER_SIZE: Int = 4096
const ERROR_CODE: Int = 0 - 1
```

### Using Constants

```aether
const MAX: Int = 100

func is_valid(x: Int) -> Int {
    if x > MAX {
        return 0
    }
    1
}
```

### Constants vs Variables

| Feature | `const` | `let` |
|---------|---------|-------|
| Mutable | No | Yes |
| When evaluated | Compile-time | Runtime |
| Can use expressions | Only literals/constants | Any expression |
| Memory | No allocation | Stack allocated |

## Type Safety

### No Implicit Conversions

Aether does not perform implicit type conversions:

```aether
let x: Int = 42
// All operations with x produce Int
let y = x + 10      // Int + Int = Int
let z = x * 2       // Int * Int = Int
```

### Type Checking

The compiler verifies types at compile time:

```aether
func takes_int(x: Int) -> Int {
    x
}

func main() -> Int {
    takes_int(42)       // OK
    // takes_int("hello")  // Would be error if strings existed
}
```

## Memory Representation

### Stack Layout

```
Stack Frame:
+------------------+
| Return Address   |
+------------------+
| Previous Frame   |
+------------------+
| Local Var 1      | <- 8 bytes (Int)
+------------------+
| Local Var 2      | <- 8 bytes (Int)
+------------------+
| ...              |
+------------------+
```

### Size of Types

| Type | Size | Alignment |
|------|------|-----------|
| Int | 8 bytes | 8 bytes |
| Bool | 8 bytes | 8 bytes |
| Pointer | 8 bytes | 8 bytes |

## Advanced: Pointers as Int

In low-level code, pointers are represented as `Int`:

```aether
let ptr = ae_malloc(1024)   // Returns Int (memory address)
ae_store64(ptr, 42)         // Store at address
let value = ae_load64(ptr)  // Load from address
```
