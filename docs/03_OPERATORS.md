# PART 3: OPERATORS

## 3.1 Arithmetic Operators

### 3.1.1 Basic Arithmetic

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `+` | Addition | `5 + 3` | `8` |
| `-` | Subtraction | `5 - 3` | `2` |
| `*` | Multiplication | `5 * 3` | `15` |
| `/` | Division | `10 / 3` | `3` |
| `%` | Modulo (remainder) | `10 % 3` | `1` |

### 3.1.2 Integer Division

Division of integers truncates toward zero:

```aether
let a = 10 / 3     // 3 (not 3.333...)
let b = -10 / 3    // -3 (truncates toward zero)
let c = 10 / -3    // -3
let d = -10 / -3   // 3
```

### 3.1.3 Modulo Behavior

The `%` operator returns the remainder:

```aether
let a = 10 % 3     // 1  (10 = 3*3 + 1)
let b = -10 % 3    // -1 (sign follows dividend)
let c = 10 % -3    // 1
let d = -10 % -3   // -1
```

### 3.1.4 Floating-Point Arithmetic

```aether
let a = 10.0 / 3.0  // 3.333...
let b = 10.5 + 2.3  // 12.8
let c = 5.0 % 2.0   // 1.0 (modulo works on floats)
```

### 3.1.5 Unary Operators

```aether
let x = 5
let negative = -x   // -5
let positive = +x   // 5 (rarely used)
```

---

## 3.2 Comparison Operators

### 3.2.1 Equality

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `==` | Equal | `5 == 5` | `true` |
| `!=` | Not equal | `5 != 3` | `true` |

```aether
let same = 5 == 5           // true
let different = 5 != 3      // true
let str_eq = "a" == "a"     // true
let str_neq = "a" != "b"    // true
```

### 3.2.2 Ordering

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `<` | Less than | `3 < 5` | `true` |
| `<=` | Less or equal | `5 <= 5` | `true` |
| `>` | Greater than | `7 > 5` | `true` |
| `>=` | Greater or equal | `5 >= 5` | `true` |

```aether
let a = 3 < 5     // true
let b = 5 <= 5    // true
let c = 7 > 5     // true
let d = 5 >= 5    // true
let e = 3 >= 5    // false
```

### 3.2.3 Chained Comparisons

Unlike some languages, Aether does NOT allow chaining:

```aether
// This does NOT work:
// let valid = 1 < x < 10

// Use logical AND instead:
let valid = x > 1 && x < 10
```

---

## 3.3 Logical Operators

### 3.3.1 Boolean Operators

| Operator | Name | Description |
|----------|------|-------------|
| `&&` | AND | True if both operands are true |
| `\|\|` | OR | True if either operand is true |
| `!` | NOT | Inverts the boolean value |

```aether
let a = true && false   // false
let b = true || false   // true
let c = !true           // false
let d = !false          // true
```

### 3.3.2 Truth Tables

**AND (`&&`):**
| A | B | A && B |
|---|---|--------|
| true | true | true |
| true | false | false |
| false | true | false |
| false | false | false |

**OR (`||`):**
| A | B | A \|\| B |
|---|---|---------|
| true | true | true |
| true | false | true |
| false | true | true |
| false | false | false |

**NOT (`!`):**
| A | !A |
|---|----|
| true | false |
| false | true |

### 3.3.3 Short-Circuit Evaluation

Logical operators use short-circuit evaluation:

```aether
// AND: if first is false, second is not evaluated
let result = false && expensive_function()  // function NOT called

// OR: if first is true, second is not evaluated
let result = true || expensive_function()   // function NOT called
```

**Practical use:**
```aether
// Safe null check pattern
if ptr != null && ptr.value > 0 {
    // ptr.value only accessed if ptr is not null
}
```

---

## 3.4 Bitwise Operators

### 3.4.1 Bit Manipulation

| Operator | Name | Description |
|----------|------|-------------|
| `&` | AND | 1 if both bits are 1 |
| `\|` | OR | 1 if either bit is 1 |
| `^` | XOR | 1 if bits are different |
| `~` | NOT | Inverts all bits |
| `<<` | Left shift | Shift bits left |
| `>>` | Right shift | Shift bits right |

### 3.4.2 Bitwise AND (`&`)

```aether
//   1100 (12)
// & 1010 (10)
// = 1000 (8)

let result = 12 & 10  // 8
```

**Use case: Check if bit is set**
```aether
let flags = 0b1010
let has_bit_1 = (flags & 0b0010) != 0  // true
```

### 3.4.3 Bitwise OR (`|`)

```aether
//   1100 (12)
// | 1010 (10)
// = 1110 (14)

let result = 12 | 10  // 14
```

**Use case: Set a bit**
```aether
let flags = 0b1000
let with_bit_1 = flags | 0b0010  // 0b1010
```

### 3.4.4 Bitwise XOR (`^`)

```aether
//   1100 (12)
// ^ 1010 (10)
// = 0110 (6)

let result = 12 ^ 10  // 6
```

**Use case: Toggle a bit**
```aether
let flags = 0b1010
let toggled = flags ^ 0b0010  // 0b1000
```

### 3.4.5 Bitwise NOT (`~`)

```aether
// ~0b00001111 = 0b11110000 (for 8-bit)
let x: u8 = 0b00001111
let inverted = ~x  // 0b11110000 (240)
```

### 3.4.6 Left Shift (`<<`)

```aether
let x = 1 << 0   // 1   (0b0001)
let y = 1 << 1   // 2   (0b0010)
let z = 1 << 2   // 4   (0b0100)
let w = 1 << 3   // 8   (0b1000)

// Multiply by power of 2
let doubled = 5 << 1   // 10 (5 * 2)
let quadrupled = 5 << 2  // 20 (5 * 4)
```

### 3.4.7 Right Shift (`>>`)

```aether
let x = 16 >> 1  // 8  (16 / 2)
let y = 16 >> 2  // 4  (16 / 4)
let z = 16 >> 3  // 2  (16 / 8)

// Divide by power of 2
let half = 100 >> 1   // 50
let quarter = 100 >> 2  // 25
```

---

## 3.5 Assignment Operators

### 3.5.1 Simple Assignment

```aether
let mut x = 5
x = 10       // x is now 10
```

### 3.5.2 Compound Assignment

| Operator | Equivalent | Example |
|----------|------------|---------|
| `+=` | `x = x + y` | `x += 5` |
| `-=` | `x = x - y` | `x -= 5` |
| `*=` | `x = x * y` | `x *= 5` |
| `/=` | `x = x / y` | `x /= 5` |
| `%=` | `x = x % y` | `x %= 5` |
| `&=` | `x = x & y` | `x &= 5` |
| `\|=` | `x = x \| y` | `x \|= 5` |
| `^=` | `x = x ^ y` | `x ^= 5` |
| `<<=` | `x = x << y` | `x <<= 2` |
| `>>=` | `x = x >> y` | `x >>= 2` |

```aether
let mut x = 10
x += 5    // x = 15
x -= 3    // x = 12
x *= 2    // x = 24
x /= 4    // x = 6
x %= 4    // x = 2
```

---

## 3.6 Operator Precedence

From highest to lowest precedence:

| Precedence | Operators | Associativity |
|------------|-----------|---------------|
| 1 (highest) | `()` (grouping) | - |
| 2 | `!`, `-` (unary), `~` | Right |
| 3 | `*`, `/`, `%` | Left |
| 4 | `+`, `-` | Left |
| 5 | `<<`, `>>` | Left |
| 6 | `&` | Left |
| 7 | `^` | Left |
| 8 | `\|` | Left |
| 9 | `<`, `<=`, `>`, `>=` | Left |
| 10 | `==`, `!=` | Left |
| 11 | `&&` | Left |
| 12 | `\|\|` | Left |
| 13 (lowest) | `=`, `+=`, etc. | Right |

### 3.6.1 Examples

```aether
// Multiplication before addition
let a = 2 + 3 * 4     // 2 + 12 = 14, not 20

// Comparison before logical
let b = 5 > 3 && 2 < 4  // (5 > 3) && (2 < 4) = true

// Use parentheses for clarity
let c = (2 + 3) * 4   // 20
let d = 2 + (3 * 4)   // 14 (same as without parens)
```

---

*Next: [Part 4: Control Flow](04_CONTROL_FLOW.md)*
