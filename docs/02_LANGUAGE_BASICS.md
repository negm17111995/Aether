# Aether Language Basics

## Lexical Structure

### Character Set
Aether source files must be UTF-8 encoded. The following characters are significant:

- **Letters**: `a-z`, `A-Z`
- **Digits**: `0-9`
- **Underscore**: `_`
- **Operators**: `+ - * / % = < > ! & |`
- **Delimiters**: `( ) { } [ ] , : .`
- **Special**: `->` (arrow)

### Identifiers

Identifiers start with a letter or underscore, followed by letters, digits, or underscores:

```aether
// Valid identifiers
x
_private
camelCase
snake_case
PascalCase
variable123
_123also_valid
```

```aether
// Invalid identifiers
123invalid    // Cannot start with digit
my-variable   // Hyphens not allowed
class         // Reserved (if it were a keyword)
```

### Keywords

Aether has minimal keywords:

| Keyword | Purpose |
|---------|---------|
| `func` | Function declaration |
| `let` | Variable declaration |
| `return` | Return from function |
| `if` | Conditional |
| `else` | Alternative branch |
| `while` | Loop |
| `const` | Constant declaration |
| `struct` | Structure definition |
| `import` | Module import |
| `Int` | Integer type |
| `Bool` | Boolean type |

### Comments

Only single-line comments are supported:

```aether
// This is a comment
func main() -> Int {
    // Comments can appear anywhere
    let x = 42  // Even at end of line
    x
}
```

### Literals

#### Integer Literals
```aether
0           // Zero
42          // Positive
123456      // Large number
// Note: Negative numbers use unary minus
0 - 42      // Negative 42
```

#### Boolean Literals
```aether
true        // Logical true (1)
false       // Logical false (0)
```

## Expressions

### Primary Expressions

```aether
42              // Integer literal
x               // Variable reference
(a + b)         // Parenthesized expression
function_name() // Function call
```

### Arithmetic Expressions

```aether
a + b           // Addition
a - b           // Subtraction
a * b           // Multiplication
a / b           // Integer division
a % b           // Modulo (remainder)
```

### Comparison Expressions

```aether
a == b          // Equal
a != b          // Not equal
a < b           // Less than
a > b           // Greater than
a <= b          // Less than or equal
a >= b          // Greater than or equal
```

### Logical Expressions

```aether
a && b          // Logical AND
a || b          // Logical OR
!a              // Logical NOT
```

### Operator Precedence (Highest to Lowest)

| Level | Operators | Associativity |
|-------|-----------|---------------|
| 1 | `!` (unary), `-` (unary) | Right |
| 2 | `*`, `/`, `%` | Left |
| 3 | `+`, `-` | Left |
| 4 | `<`, `>`, `<=`, `>=` | Left |
| 5 | `==`, `!=` | Left |
| 6 | `&&` | Left |
| 7 | `\|\|` | Left |

### Expression Examples

```aether
// Precedence demonstration
1 + 2 * 3           // = 7 (not 9)
(1 + 2) * 3         // = 9

// Complex expression
a + b * c - d / e   // Same as: a + (b * c) - (d / e)

// Logical with arithmetic
x > 0 && y < 10     // Same as: (x > 0) && (y < 10)

// Chained comparisons need explicit &&
a < b && b < c      // a < b < c (conceptually)
```

## Statements

### Expression Statements

Any expression can be a statement:

```aether
42              // Expression statement (useful in blocks)
x + y           // Computed but result unused
some_function() // Side effects
```

### Variable Declaration

```aether
let variable_name = expression
let variable_name: Type = expression
```

Examples:
```aether
let x = 42
let y: Int = 100
let result = compute_something()
```

### Assignment

```aether
variable_name = expression
```

Examples:
```aether
x = 10
y = y + 1
counter = counter * 2
```

### Return Statement

```aether
return expression
```

Examples:
```aether
return 42
return x + y
return compute()
```

### Block Statements

Enclosed in `{ }`, last expression is the value:

```aether
{
    let a = 10
    let b = 20
    a + b       // Block evaluates to 30
}
```

## Program Structure

### Minimal Program

```aether
func main() -> Int {
    0
}
```

### Complete Program Structure

```aether
// Optional: module imports
import std
import std.io

// Optional: constants
const MAX_SIZE: Int = 1024
const PI_APPROX: Int = 3

// Optional: helper functions
func helper(x: Int) -> Int {
    x * 2
}

// Required: main function
func main() -> Int {
    let result = helper(21)
    result  // Exit code
}
```

### Execution Flow

1. Program starts at `main()`
2. Statements execute top-to-bottom
3. Return value of `main()` becomes exit code
4. Exit codes 0-255 are meaningful (Unix convention)
