# PART 8: MEMORY MANAGEMENT

## 8.1 Ownership System

### 8.1.1 What is Ownership?

Aether uses **ownership** to manage memory without garbage collection:

- Every value has exactly one owner
- When the owner goes out of scope, the value is dropped
- Ownership can be transferred (moved)

### 8.1.2 Basic Ownership

```aether
func main() {
    let s = "Hello"  // s owns the string
    
    // s is valid here
    print(s)
    
}  // s goes out of scope, memory is freed
```

### 8.1.3 Move Semantics

Ownership transfers on assignment:

```aether
let s1 = "Hello"
let s2 = s1  // s1 MOVED to s2

// print(s1)  // ERROR: s1 is no longer valid
print(s2)    // OK: s2 owns the string
```

**Why?** This prevents double-free bugs:
```aether
// Without moves, both s1 and s2 would try to free the same memory
```

### 8.1.4 Copy Types

Simple types are copied, not moved:

```aether
let x = 5
let y = x  // x is COPIED to y

print(x)  // OK: 5
print(y)  // OK: 5
```

**Copy types:**
- All integers (`Int`, `i32`, `u8`, etc.)
- Floating point (`Float`, `f32`, `f64`)
- Boolean (`Bool`)
- Character (`Char`)
- Tuples of copy types

### 8.1.5 Functions and Ownership

Passing to function transfers ownership:

```aether
func take_string(s: String) {
    print(s)
}  // s is dropped here

func main() {
    let s = "Hello"
    take_string(s)  // s moved into function
    
    // print(s)  // ERROR: s is no longer valid
}
```

Returning transfers ownership out:

```aether
func give_string() -> String {
    let s = "Hello"
    s  // Ownership transferred to caller
}

func main() {
    let s = give_string()  // s now owns the string
    print(s)  // OK
}
```

---

## 8.2 Borrowing (References)

### 8.2.1 What is Borrowing?

Borrowing lets you use a value without taking ownership:

```aether
func calculate_length(s: &String) -> Int {
    s.len()  // Can read s
}  // s goes out of scope, but doesn't drop the value (it's borrowed)

func main() {
    let s = "Hello, World!"
    let len = calculate_length(&s)  // Borrow s
    
    print(s)    // s is still valid!
    print(len)  // 13
}
```

### 8.2.2 Reference Syntax

```aether
let x = 5
let r = &x     // Create reference to x
print(*r)      // Dereference: 5
print(r)       // Also works: 5 (auto-deref)
```

### 8.2.3 Mutable References

```aether
func add_exclaim(s: &mut String) {
    s.push('!')  // Modify the borrowed value
}

func main() {
    let mut s = "Hello"
    add_exclaim(&mut s)
    print(s)  // "Hello!"
}
```

### 8.2.4 Borrowing Rules

**Rule 1:** You can have EITHER:
- One mutable reference, OR
- Any number of immutable references

```aether
let mut s = "Hello"

// Multiple immutable refs: OK
let r1 = &s
let r2 = &s
let r3 = &s
print(f"{r1}, {r2}, {r3}")

// One mutable ref: OK
let r = &mut s
r.push('!')

// CANNOT have both:
let r1 = &s        // immutable borrow
let r2 = &mut s    // ERROR: cannot borrow mutably while immutably borrowed
```

**Rule 2:** References must be valid (no dangling pointers)

```aether
func dangling() -> &String {
    let s = "Hello"
    &s  // ERROR: s will be dropped, can't return reference
}

func valid() -> String {
    let s = "Hello"
    s  // OK: return owned value
}
```

### 8.2.5 Reference Lifetimes

References have lifetimes that the compiler tracks:

```aether
func longest<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` means: "the returned reference lives as long as the shorter of x and y"

---

## 8.3 Slices

### 8.3.1 What are Slices?

Slices are references to a portion of data:

```aether
let arr = [1, 2, 3, 4, 5]
let slice = &arr[1..4]  // Reference to [2, 3, 4]

print(slice.len())  // 3
print(slice[0])     // 2
```

### 8.3.2 String Slices

```aether
let s = "Hello, World!"
let hello = &s[0..5]    // "Hello"
let world = &s[7..12]   // "World"
```

### 8.3.3 Slice Syntax

```aether
let arr = [0, 1, 2, 3, 4, 5]

let a = &arr[2..5]   // [2, 3, 4]
let b = &arr[..3]    // [0, 1, 2]
let c = &arr[3..]    // [3, 4, 5]
let d = &arr[..]     // [0, 1, 2, 3, 4, 5]
```

---

## 8.4 Smart Pointers

### 8.4.1 Box<T> - Heap Allocation

```aether
// Allocate on heap
let boxed = Box::new(42)
print(*boxed)  // 42

// Useful for:
// - Large data (avoid stack overflow)
// - Recursive types
// - Trait objects

struct List {
    value: Int,
    next: Option<Box<List>>,  // Recursive type needs Box
}
```

### 8.4.2 Rc<T> - Reference Counting

```aether
use std::rc::Rc

// Shared ownership
let a = Rc::new(5)
let b = Rc::clone(&a)
let c = Rc::clone(&a)

print(Rc::count(&a))  // 3

// All references share the same data
print(*a)  // 5
print(*b)  // 5
print(*c)  // 5

// When all Rc's are dropped, data is freed
```

### 8.4.3 RefCell<T> - Interior Mutability

```aether
use std::cell::RefCell

let cell = RefCell::new(5)

// Borrow mutably at runtime
{
    let mut borrow = cell.borrow_mut()
    *borrow = 10
}

print(*cell.borrow())  // 10
```

### 8.4.4 Arc<T> - Thread-Safe Reference Counting

```aether
use std::sync::Arc

let data = Arc::new([1, 2, 3, 4, 5])

// Share across threads
spawn(|| {
    let local = Arc::clone(&data)
    print(local[0])
})
```

---

## 8.5 Manual Memory (Unsafe)

### 8.5.1 Raw Pointers

```aether
unsafe {
    let x = 42
    let ptr = &x as *const Int  // Raw pointer
    
    print(*ptr)  // Dereference (unsafe!)
}
```

### 8.5.2 malloc/free

```aether
unsafe {
    // Allocate 1024 bytes
    let ptr = __builtin_malloc(1024)
    
    // Use the memory
    __builtin_store64(ptr, 42)
    let value = __builtin_load64(ptr)
    
    // Free the memory
    __builtin_free(ptr)
}
```

### 8.5.3 Safety Guarantees Disabled

In `unsafe` blocks, you promise:
- Pointers are valid
- No data races
- Memory is properly aligned

---

## 8.6 Memory Layout

### 8.6.1 Stack vs Heap

**Stack:**
- Fast allocation/deallocation
- Fixed size, known at compile time
- Local variables, function arguments

**Heap:**
- Slower, requires allocation
- Dynamic size
- Box, Vec, String contents

### 8.6.2 Struct Layout

```aether
struct Point {
    x: Int,  // 8 bytes
    y: Int,  // 8 bytes
}
// Total: 16 bytes, no padding needed

struct Mixed {
    a: u8,   // 1 byte
    // 7 bytes padding
    b: Int,  // 8 bytes
    c: u8,   // 1 byte
    // 7 bytes padding
}
// Total: 24 bytes due to alignment
```

### 8.6.3 Optimized Layout

```aether
#[repr(packed)]
struct Compact {
    a: u8,
    b: u8,
    c: u8,
}
// Total: 3 bytes, no padding

#[repr(C)]
struct CCompatible {
    x: Int,
    y: Int,
}
// Layout matches C struct
```

---

*Next: [Part 9: Concurrency](09_CONCURRENCY.md)*
