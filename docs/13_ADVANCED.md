# PART 13: ADVANCED TOPICS

## 13.1 Macros

### 13.1.1 What are Macros?

Macros generate code at compile time:
- Reduce boilerplate
- Create DSLs
- Metaprogramming

### 13.1.2 Declarative Macros

```aether
// Define macro
macro vec!($($x:expr),*) {
    {
        let mut v = Vec::new()
        $(v.push($x);)*
        v
    }
}

// Usage
let v = vec![1, 2, 3, 4, 5]
```

### 13.1.3 Debug Print Macro

```aether
macro dbg!($x:expr) {
    {
        let val = $x
        eprintln(f"{stringify!($x)} = {val:?}")
        val
    }
}

// Usage
let x = 5
let y = dbg!(x * 2)  // Prints: x * 2 = 10
// y = 10
```

### 13.1.4 Assert Macros

```aether
macro assert!($cond:expr) {
    if !$cond {
        panic(f"assertion failed: {stringify!($cond)}")
    }
}

macro assert_eq!($left:expr, $right:expr) {
    let l = $left
    let r = $right
    if l != r {
        panic(f"assertion failed: {stringify!($left)} == {stringify!($right)}\n  left: {l:?}\n right: {r:?}")
    }
}

// Usage
assert!(x > 0)
assert_eq!(2 + 2, 4)
```

### 13.1.5 Macro Patterns

| Pattern | Matches |
|---------|---------|
| `$x:expr` | Expression |
| `$t:ty` | Type |
| `$i:ident` | Identifier |
| `$p:pat` | Pattern |
| `$b:block` | Block |
| `$s:stmt` | Statement |
| `$($x),*` | Zero or more |
| `$($x),+` | One or more |
| `$($x)?` | Optional |

---

## 13.2 Attributes

### 13.2.1 Function Attributes

```aether
// Inline hint
#[inline]
func fast_func() -> Int { 42 }

// Always inline
#[inline(always)]
func critical_func() -> Int { 42 }

// Never inline
#[inline(never)]
func cold_path() { ... }

// Mark unused
#[allow(unused)]
func experimental() { ... }
```

### 13.2.2 Derive Attributes

```aether
#[derive(Debug, Clone, Eq, Ord)]
struct Point {
    x: Int,
    y: Int,
}

// Now Point has:
// - Debug formatting
// - clone() method
// - == and != operators
// - < > <= >= operators
```

### 13.2.3 Conditional Compilation

```aether
#[cfg(target_os = "macos")]
func platform_specific() {
    // Only compiled on macOS
}

#[cfg(target_arch = "arm64")]
func arm_optimized() {
    // Only compiled on ARM64
}

#[cfg(debug)]
func debug_only() {
    // Only in debug builds
}

#[cfg(test)]
mod tests {
    // Only compiled for tests
}
```

### 13.2.4 Documentation Attributes

```aether
/// This is documentation for the function.
/// 
/// # Examples
/// 
/// ```
/// let x = add(2, 3)
/// assert_eq!(x, 5)
/// ```
pub func add(a: Int, b: Int) -> Int {
    a + b
}

#[doc(hidden)]
func internal_impl() { ... }
```

### 13.2.5 Deprecation

```aether
#[deprecated(since = "1.5.0", note = "Use new_function instead")]
func old_function() {
    // ...
}
```

---

## 13.3 Unsafe Code

### 13.3.1 When to Use Unsafe

Unsafe code is needed for:
- FFI (Foreign Function Interface)
- Raw pointer manipulation
- Inline assembly
- Performance-critical code

### 13.3.2 Unsafe Blocks

```aether
let x = 42

unsafe {
    // Raw pointer operations
    let ptr = &x as *const Int
    let value = *ptr  // Dereference raw pointer
}
```

### 13.3.3 Unsafe Functions

```aether
unsafe func dangerous_operation(ptr: *mut Int) {
    *ptr = 42  // Direct memory write
}

// Must call in unsafe block
unsafe {
    dangerous_operation(some_ptr)
}
```

### 13.3.4 Raw Memory Operations

```aether
unsafe {
    // Allocate
    let ptr = __builtin_malloc(1024)
    
    // Write
    __builtin_store64(ptr, 42)
    __builtin_store32(ptr + 8, 100)
    __builtin_store8(ptr + 12, 255)
    
    // Read
    let a = __builtin_load64(ptr)     // 42
    let b = __builtin_load32(ptr + 8)  // 100
    let c = __builtin_load8(ptr + 12)  // 255
    
    // Copy
    __builtin_memcpy(dest, src, 100)
    
    // Free
    __builtin_free(ptr)
}
```

### 13.3.5 FFI (Foreign Function Interface)

```aether
#[extern("C")]
func strlen(s: *const u8) -> Int

#[extern("C")]
func malloc(size: Int) -> *mut u8

// Usage
unsafe {
    let s = "hello\0".as_ptr()
    let len = strlen(s)  // 5
}
```

---

## 13.4 Testing

### 13.4.1 Unit Tests

```aether
func add(a: Int, b: Int) -> Int {
    a + b
}

#[test]
func test_add() {
    assert_eq!(add(2, 3), 5)
    assert_eq!(add(-1, 1), 0)
    assert_eq!(add(0, 0), 0)
}

#[test]
func test_add_negative() {
    assert_eq!(add(-5, -3), -8)
}
```

### 13.4.2 Running Tests

```bash
# Run all tests
./aetherc test

# Run specific test
./aetherc test test_add

# Verbose output
./aetherc test --verbose
```

### 13.4.3 Test Assertions

```aether
// Equality
assert_eq!(actual, expected)
assert_ne!(actual, unexpected)

// Boolean
assert!(condition)
assert!(!condition)

// Comparison
assert!(x > 0)
assert!(x >= 0)
assert!(x < 100)
```

### 13.4.4 Testing Panics

```aether
#[test]
#[should_panic]
func test_divide_by_zero() {
    divide(1, 0)  // Should panic
}

#[test]
#[should_panic(expected = "division by zero")]
func test_divide_by_zero_message() {
    divide(1, 0)
}
```

### 13.4.5 Test Organization

```aether
// In same file
#[cfg(test)]
mod tests {
    use super::*
    
    #[test]
    func test_function() { ... }
}

// In separate test file: tests/test_math.aether
use mylib::math

#[test]
func test_add() {
    assert_eq!(math::add(2, 3), 5)
}
```

---

## 13.5 Benchmarking

### 13.5.1 Basic Benchmarks

```aether
#[bench]
func bench_sort(b: &mut Bencher) {
    let data = vec![5, 3, 1, 4, 2]
    
    b.iter(|| {
        let mut v = data.clone()
        v.sort()
    })
}
```

### 13.5.2 Running Benchmarks

```bash
./aetherc bench

# Output:
# bench_sort ... 1,234 ns/iter (+/- 50)
```

---

## 13.6 Documentation

### 13.6.1 Doc Comments

```aether
/// Adds two integers.
/// 
/// # Arguments
/// 
/// * `a` - First integer
/// * `b` - Second integer
/// 
/// # Returns
/// 
/// The sum of `a` and `b`
/// 
/// # Examples
/// 
/// ```
/// let sum = add(2, 3)
/// assert_eq!(sum, 5)
/// ```
/// 
/// # Panics
/// 
/// This function never panics.
pub func add(a: Int, b: Int) -> Int {
    a + b
}
```

### 13.6.2 Module Documentation

```aether
//! This module provides mathematical utilities.
//! 
//! # Overview
//! 
//! The math module includes:
//! - Basic arithmetic functions
//! - Trigonometric functions
//! - Statistical functions
//! 
//! # Examples
//! 
//! ```
//! use std::math
//! 
//! let x = math::sqrt(16.0)
//! assert_eq!(x, 4.0)
//! ```

pub func sqrt(x: Float) -> Float { ... }
```

### 13.6.3 Generating Documentation

```bash
./aetherc doc

# Opens generated documentation in browser
./aetherc doc --open
```

---

## 13.7 Compiler Intrinsics

### 13.7.1 Memory Intrinsics

```aether
// Allocation
__builtin_malloc(size: Int) -> Int
__builtin_free(ptr: Int)

// Memory operations
__builtin_memcpy(dst: Int, src: Int, len: Int)
__builtin_memset(ptr: Int, value: u8, len: Int)

// Load/Store
__builtin_load64(ptr: Int) -> Int
__builtin_load32(ptr: Int) -> i32
__builtin_load8(ptr: Int) -> i8
__builtin_store64(ptr: Int, val: Int)
__builtin_store32(ptr: Int, val: i32)
__builtin_store8(ptr: Int, val: i8)
```

### 13.7.2 IO Intrinsics

```aether
__builtin_print(s: Int)
__builtin_println(s: Int)
__builtin_read_line() -> Int
```

### 13.7.3 System Intrinsics

```aether
__builtin_exit(code: Int)
__builtin_abort()
```

---

*Next: [Part 14: Best Practices](14_BEST_PRACTICES.md)*
