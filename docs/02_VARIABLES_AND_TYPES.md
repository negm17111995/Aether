# PART 2: VARIABLES AND TYPES

## 2.1 Variables

### 2.1.1 Declaring Variables with `let`

Variables are declared using the `let` keyword:

```aether
let x = 5
let name = "Alice"
let pi = 3.14159
```

**Key Points:**
- Variables are **immutable by default** (cannot be changed after creation)
- The type is **inferred** from the value
- No semicolon needed at line end

### 2.1.2 Why Immutable by Default?

Immutability prevents bugs:

```aether
let x = 5
x = 10  // ERROR: Cannot assign to immutable variable 'x'
```

This catches accidental modifications at compile time.

### 2.1.3 Mutable Variables with `mut`

When you need to change a variable, use `mut`:

```aether
let mut counter = 0
counter = counter + 1  // OK
counter = 10           // OK
print(counter)         // 10
```

### 2.1.4 Variable Shadowing

You can redeclare a variable with the same name:

```aether
let x = 5
print(x)      // 5

let x = "now a string"
print(x)      // now a string

let x = x + "!"
print(x)      // now a string!
```

**Why use shadowing?**
- Transform a value without needing a new name
- Change the type of a value
- Mark transformations clearly in code

### 2.1.5 Explicit Type Annotations

You can specify the type explicitly:

```aether
let x: Int = 5
let name: String = "Alice"
let pi: Float = 3.14159
let active: Bool = true
```

**When to use explicit types:**
- When the type cannot be inferred
- When you want to be explicit for documentation
- When you want a different type than inferred

Example:
```aether
let x = 5           // Inferred as Int
let y: Float = 5    // Explicitly Float (5.0)
```

---

## 2.2 Constants

### 2.2.1 Declaring Constants with `const`

Constants are compile-time values:

```aether
const MAX_SIZE: Int = 1024
const PI: Float = 3.14159265359
const APP_NAME: String = "MyApp"
```

**Rules for constants:**
- Must have explicit type annotation
- Must be known at compile time
- Cannot be computed from variables
- Conventionally use `SCREAMING_SNAKE_CASE`

### 2.2.2 Constants vs. Variables

| Feature | `const` | `let` |
|---------|---------|-------|
| When evaluated | Compile time | Runtime |
| Type annotation | Required | Optional |
| Can be mutable | Never | With `mut` |
| Can use expressions | Only constant expressions | Any expression |

```aether
const SIZE: Int = 100          // OK: constant value
const DOUBLED: Int = SIZE * 2  // OK: constant expression

let x = 5
const FROM_VAR: Int = x        // ERROR: cannot use variable in const
```

---

## 2.3 Primitive Types

### 2.3.1 Integer Types

| Type | Bits | Range | Example |
|------|------|-------|---------|
| `Int` | 64 | -2^63 to 2^63-1 | `42`, `-17` |
| `i8` | 8 | -128 to 127 | `let x: i8 = 100` |
| `i16` | 16 | -32,768 to 32,767 | `let x: i16 = 1000` |
| `i32` | 32 | -2^31 to 2^31-1 | `let x: i32 = 100000` |
| `i64` | 64 | -2^63 to 2^63-1 | `let x: i64 = 100000000` |
| `u8` | 8 | 0 to 255 | `let x: u8 = 200` |
| `u16` | 16 | 0 to 65,535 | `let x: u16 = 50000` |
| `u32` | 32 | 0 to 2^32-1 | `let x: u32 = 4000000000` |
| `u64` | 64 | 0 to 2^64-1 | `let x: u64 = 10000000000` |

**Default integer type is `Int` (i64):**
```aether
let x = 42  // Type is Int (i64)
```

### 2.3.2 Integer Literals

```aether
// Decimal (base 10)
let decimal = 1_000_000     // Underscores for readability

// Hexadecimal (base 16)
let hex = 0xFF              // 255
let hex2 = 0xDEAD_BEEF      // With underscores

// Octal (base 8)
let octal = 0o777           // 511

// Binary (base 2)
let binary = 0b1010_1010    // 170

// Type suffixes
let byte = 255u8            // u8 type
let long = 1000000i64       // i64 type
```

### 2.3.3 Floating-Point Types

| Type | Bits | Precision | Example |
|------|------|-----------|---------|
| `Float` | 64 | ~15-17 digits | `3.14159` |
| `f32` | 32 | ~6-9 digits | `let x: f32 = 3.14` |
| `f64` | 64 | ~15-17 digits | `let x: f64 = 3.14159` |

```aether
let x = 3.14          // Float (f64)
let y = 2.5e10        // Scientific notation: 25000000000
let z = 1.5e-3        // 0.0015
```

### 2.3.4 Boolean Type

```aether
let yes: Bool = true
let no: Bool = false

// From comparisons
let is_greater = 5 > 3   // true
let is_equal = 5 == 5    // true
```

### 2.3.5 Character Type

```aether
let letter: Char = 'a'
let digit: Char = '7'
let emoji: Char = '🚀'
let unicode: Char = '\u{1F600}'  // 😀
```

**Escape sequences:**
| Escape | Meaning |
|--------|---------|
| `\n` | Newline |
| `\t` | Tab |
| `\r` | Carriage return |
| `\\` | Backslash |
| `\'` | Single quote |
| `\"` | Double quote |
| `\0` | Null character |
| `\u{XXXX}` | Unicode codepoint |

### 2.3.6 String Type

```aether
let greeting: String = "Hello, World!"
let empty: String = ""
let multiline = "Line 1\nLine 2\nLine 3"
```

**String operations:**
```aether
let s = "Hello"
let len = s.len()         // 5
let ch = s.char_at(0)     // 'H'
let sub = s.slice(0, 3)   // "Hel"
let upper = s.to_upper()  // "HELLO"
let lower = s.to_lower()  // "hello"
```

**String concatenation:**
```aether
let a = "Hello"
let b = " World"
let c = a + b             // "Hello World"
```

**String interpolation:**
```aether
let name = "Alice"
let age = 30
let msg = f"Name: {name}, Age: {age}"
// "Name: Alice, Age: 30"

// Expressions in interpolation
let calc = f"2 + 2 = {2 + 2}"
// "2 + 2 = 4"
```

---

## 2.4 Type Conversion

### 2.4.1 Explicit Conversion

Aether requires explicit type conversion:

```aether
let x: Int = 5
let y: Float = x as Float   // Convert Int to Float

let a: Float = 3.7
let b: Int = a as Int       // Convert Float to Int (truncates: 3)

let c: i32 = 100
let d: i64 = c as i64       // Widen i32 to i64
```

### 2.4.2 Why No Implicit Conversion?

Implicit conversions hide bugs:

```aether
// In C:
int x = 3000000000;    // Silently overflows on 32-bit int

// In Aether:
let x: i32 = 3000000000  // Compile ERROR: value too large for i32
```

---

## 2.5 Type Aliases

Create new names for types:

```aether
type UserId = Int
type Point2D = (Int, Int)
type Handler = fn(Int) -> Bool

let id: UserId = 42
let pos: Point2D = (10, 20)
```

---

*Next: [Part 3: Operators](03_OPERATORS.md)*
