# Aether Programming Language

> **The World's First Truly Universal Programming Language**
> Self-hosted • Zero Dependencies • Native Performance • Cross-Platform

---

## Table of Contents

1. [Overview](#overview)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Language Fundamentals](#language-fundamentals)
5. [Data Types](#data-types)
6. [Variables & Constants](#variables--constants)
7. [Operators](#operators)
8. [Control Flow](#control-flow)
9. [Functions](#functions)
10. [Memory Management](#memory-management)
11. [Collections](#collections)
12. [Modules & Imports](#modules--imports)
13. [Error Handling](#error-handling)
14. [Concurrency](#concurrency)
15. [Veritas Features](#veritas-features)
16. [Native Compilation](#native-compilation)
17. [Standard Library](#standard-library)
18. [Best Practices](#best-practices)
19. [Examples](#examples)

---

## Overview

Aether is a **100% self-hosted**, **pure native** programming language designed for:

- **Performance**: Compiles directly to ARM64/x86_64 machine code
- **Safety**: Memory-safe with ownership semantics
- **Simplicity**: Clean, minimal syntax
- **Universality**: One language for systems, web, cloud, and AI

### Key Features

| Feature | Description |
|---------|-------------|
| **Native Compilation** | Direct to Mach-O (macOS), ELF (Linux), PE (Windows) |
| **Self-Hosted** | Compiler written in Aether, compiles itself |
| **Zero C Dependencies** | No libc, no external libraries required |
| **Veritas System** | Compile-time verification, dependent types, actors |
| **Morphic Runtime** | Adaptive JIT-like optimization |
| **Cross-Platform** | macOS ARM64/x64, Linux x64/ARM64 |

---

## Installation

### From Source

```bash
git clone https://github.com/your-repo/aether-core.git
cd aether-core
./bootstrap/aetherc_native --version
```

### Verify Installation

```bash
echo 'func main() -> Int { 42 }' > hello.aether
./bootstrap/aetherc_native hello.aether
./a.out
# Output: exits with code 42
```

---

## Quick Start

### Hello World

```aether
func main() -> Int {
    42
}
```

### Compile & Run

```bash
./bootstrap/aetherc_native program.aether
./a.out
```

### Cross-Compilation

```bash
# For Linux x64
./bootstrap/aetherc_native program.aether --target linux-x64 -o server

# For macOS ARM64 (default on Apple Silicon)
./bootstrap/aetherc_native program.aether --target macos-arm64
```

---

## Language Fundamentals

### Comments

```aether
// Single-line comment

// Multi-line comments use multiple single-line comments
// There is no /* */ style comment
```

### Statements

Every statement is an expression. The last expression in a block is the return value:

```aether
func example() -> Int {
    let x = 10
    let y = 20
    x + y  // This is returned (30)
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

---

## Data Types

### Primitive Types

| Type | Description | Example |
|------|-------------|---------|
| `Int` | 64-bit signed integer | `42`, `-100`, `0` |
| `Bool` | Boolean (true/false) | `true`, `false` |

### Type Annotations

```aether
let x: Int = 42
let flag: Bool = true
```

### Type Inference

Types are inferred when possible:

```aether
let x = 42        // Inferred as Int
let y = x + 10    // Inferred as Int
```

---

## Variables & Constants

### Variables with `let`

```aether
let x = 10        // Declare and initialize
x = 20            // Reassign
x = x + 1         // Increment
```

### Multiple Variables

```aether
let a = 1
let b = 2
let c = 3
let sum = a + b + c
```

### Shadowing

```aether
let x = 10
let x = x + 5     // New x shadows old x, now 15
```

---

## Operators

### Arithmetic Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `+` | Addition | `10 + 5` → `15` |
| `-` | Subtraction | `10 - 5` → `5` |
| `*` | Multiplication | `10 * 5` → `50` |
| `/` | Division | `10 / 5` → `2` |
| `%` | Modulo | `10 % 3` → `1` |

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equal | `5 == 5` → `true` |
| `!=` | Not equal | `5 != 3` → `true` |
| `<` | Less than | `3 < 5` → `true` |
| `>` | Greater than | `5 > 3` → `true` |
| `<=` | Less or equal | `5 <= 5` → `true` |
| `>=` | Greater or equal | `5 >= 3` → `true` |

### Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `&&` | Logical AND | `true && false` → `false` |
| `\|\|` | Logical OR | `true \|\| false` → `true` |
| `!` | Logical NOT | `!true` → `false` |

### Operator Precedence (Highest to Lowest)

1. `!` (unary NOT)
2. `*`, `/`, `%`
3. `+`, `-`
4. `<`, `>`, `<=`, `>=`
5. `==`, `!=`
6. `&&`
7. `||`

---

## Control Flow

### If/Else

```aether
func check(x: Int) -> Int {
    if x > 0 {
        return 1
    } else {
        return 0
    }
}
```

### Nested Conditions

```aether
func grade(score: Int) -> Int {
    if score >= 90 {
        return 4  // A
    } else {
        if score >= 80 {
            return 3  // B
        } else {
            if score >= 70 {
                return 2  // C
            } else {
                return 1  // D/F
            }
        }
    }
}
```

### While Loops

```aether
func sum_to_n(n: Int) -> Int {
    let sum = 0
    let i = 1
    while i <= n {
        sum = sum + i
        i = i + 1
    }
    sum
}
```

### Loop Control

```aether
func find_first_divisible(n: Int, divisor: Int) -> Int {
    let i = 1
    while i <= n {
        if i % divisor == 0 {
            return i  // Early exit
        }
        i = i + 1
    }
    0  // Not found
}
```

---

## Functions

### Function Declaration

```aether
func function_name(param1: Type1, param2: Type2) -> ReturnType {
    // body
}
```

### Examples

```aether
// No parameters, returns Int
func get_magic_number() -> Int {
    42
}

// One parameter
func double(x: Int) -> Int {
    x * 2
}

// Multiple parameters
func add(a: Int, b: Int) -> Int {
    a + b
}

// Calling functions
func main() -> Int {
    let x = get_magic_number()
    let y = double(x)
    add(x, y)
}
```

### Recursive Functions

```aether
func factorial(n: Int) -> Int {
    if n <= 1 {
        return 1
    }
    n * factorial(n - 1)
}

func fibonacci(n: Int) -> Int {
    if n <= 1 {
        return n
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
```

### Higher-Order Patterns

```aether
func apply_twice(x: Int) -> Int {
    let y = double(x)
    double(y)
}
```

---

## Memory Management

### Stack Allocation

All local variables are stack-allocated by default:

```aether
func example() -> Int {
    let x = 10    // Stack allocated
    let y = 20    // Stack allocated
    x + y         // Automatic cleanup on function exit
}
```

### Heap Allocation

Use `ae_malloc` for dynamic allocation:

```aether
func create_buffer(size: Int) -> Int {
    let ptr = ae_malloc(size)
    ptr
}
```

### Memory Operations

```aether
// Store 8-bit value
ae_store8(addr, value)

// Load 8-bit value
let byte = ae_load8(addr)

// Store 64-bit value
ae_store64(addr, value)

// Load 64-bit value
let num = ae_load64(addr)
```

### Manual Memory Management Pattern

```aether
func work_with_buffer() -> Int {
    let buf = ae_malloc(1024)
    
    // Use buffer
    ae_store64(buf, 42)
    ae_store64(buf + 8, 100)
    
    let result = ae_load64(buf) + ae_load64(buf + 8)
    
    // Note: No automatic free - manage manually
    result
}
```

---

## Collections

### Vectors (Dynamic Arrays)

```aether
func vector_example() -> Int {
    let v = vec_new()
    
    vec_push(v, 10)
    vec_push(v, 20)
    vec_push(v, 30)
    
    let len = vec_len(v)      // 3
    let first = vec_get(v, 0) // 10
    let last = vec_get(v, 2)  // 30
    
    first + last  // 40
}
```

### Vector Operations

| Function | Description |
|----------|-------------|
| `vec_new()` | Create empty vector |
| `vec_push(v, val)` | Add element to end |
| `vec_get(v, index)` | Get element at index |
| `vec_set(v, index, val)` | Set element at index |
| `vec_len(v)` | Get length |

### Building Strings

```aether
func make_hello() -> Int {
    let s = ae_malloc(6)
    ae_store8(s, 72)     // 'H'
    ae_store8(s + 1, 101) // 'e'
    ae_store8(s + 2, 108) // 'l'
    ae_store8(s + 3, 108) // 'l'
    ae_store8(s + 4, 111) // 'o'
    ae_store8(s + 5, 0)   // null terminator
    s
}
```

---

## Modules & Imports

### Import Syntax

```aether
import std
import std.io
import std.collections
import aether_compiler.lexer
```

### Module Structure

```
stdlib/
├── std.aether              # Core standard library
├── lib.aether              # Master import file
├── std/
│   ├── io.aether           # I/O operations
│   ├── net.aether          # Networking
│   ├── collections.aether  # Data structures
│   ├── runtime/
│   │   ├── vec.aether      # Vector implementation
│   │   ├── map.aether      # HashMap implementation
│   │   └── str.aether      # String operations
│   └── ...
└── aether_compiler/
    ├── lexer.aether
    ├── parser.aether
    └── native/
        ├── arm64.aether
        └── x86_64.aether
```

### Creating Modules

Each `.aether` file is a module. Export functions by defining them:

```aether
// mymodule.aether
func public_function(x: Int) -> Int {
    x * 2
}

func helper(x: Int) -> Int {
    x + 1
}
```

---

## Error Handling

### Return Codes

Use return values for error handling:

```aether
const SUCCESS: Int = 0
const ERROR_NOT_FOUND: Int = 1
const ERROR_INVALID: Int = 2

func find_item(id: Int) -> Int {
    if id < 0 {
        return ERROR_INVALID
    }
    if id > 1000 {
        return ERROR_NOT_FOUND
    }
    SUCCESS
}
```

### Optional Pattern

```aether
const NONE: Int = 0 - 1

func safe_divide(a: Int, b: Int) -> Int {
    if b == 0 {
        return NONE
    }
    a / b
}

func main() -> Int {
    let result = safe_divide(10, 2)
    if result == NONE {
        return 1  // Error
    }
    result  // 5
}
```

### Result Pattern

```aether
// Result: [is_ok, value]
func result_ok(value: Int) -> Int {
    let r = ae_malloc(16)
    ae_store64(r, 1)      // is_ok = true
    ae_store64(r + 8, value)
    r
}

func result_err(code: Int) -> Int {
    let r = ae_malloc(16)
    ae_store64(r, 0)      // is_ok = false
    ae_store64(r + 8, code)
    r
}

func is_ok(r: Int) -> Int {
    ae_load64(r)
}

func unwrap(r: Int) -> Int {
    ae_load64(r + 8)
}
```

---

## Concurrency

### Parallel Execution

```aether
import std.runtime.parallel

func parallel_sum(data: Int, len: Int) -> Int {
    parallel_reduce(data, len, 0, add)
}
```

### Async Operations

```aether
import std.runtime.async

func async_fetch(url: Int) -> Int {
    async_start(fetch_data, url)
}

func await_result(handle: Int) -> Int {
    async_wait(handle)
}
```

### Actor Model

```aether
import std.actor.actor

func actor_example() -> Int {
    let actor = actor_spawn()
    actor_send(actor, 42)
    let msg = actor_recv(actor)
    msg
}
```

---

## Veritas Features

### 1. Hardware-Aware Comptime

Compile-time CPU detection and optimization:

```aether
import std.comptime.hw

func optimized_compute() -> Int {
    let cpu = detect_cpu_arch()
    let simd = detect_simd_capability()
    
    if simd == SIMD_NEON {
        // Use ARM NEON optimizations
        return neon_fast_path()
    }
    if simd == SIMD_AVX2 {
        // Use x86 AVX2 optimizations
        return avx2_fast_path()
    }
    // Fallback
    generic_path()
}
```

### 2. Dependent Types

Types that depend on values:

```aether
import aether_compiler.veritas.dependent

// Array with compile-time size checking
func bounded_array_example() -> Int {
    let arr = bounded_new(10)  // Max 10 elements
    
    bounded_push(arr, 1)
    bounded_push(arr, 2)
    bounded_push(arr, 3)
    
    bounded_get(arr, 1)  // 2
}
```

### 3. Actor Model

Message-passing concurrency:

```aether
import std.actor.actor

func spawn_workers() -> Int {
    let worker1 = actor_spawn()
    let worker2 = actor_spawn()
    
    actor_send(worker1, 100)
    actor_send(worker2, 200)
    
    let r1 = actor_recv(worker1)
    let r2 = actor_recv(worker2)
    
    r1 + r2
}
```

### 4. Algebraic Effects

Structured effect handling:

```aether
import std.effects.effects

func safe_divide(a: Int, b: Int) -> Int {
    if b == 0 {
        effect_raise(ERROR_DIVISION_BY_ZERO)
        return 999  // Error value
    }
    a / b
}

func with_error_handling() -> Int {
    let handler = effect_handler_new()
    effect_push(handler)
    
    let result = safe_divide(100, 0)
    
    effect_pop()
    result  // 999 if error
}
```

### 5. Bit-Level FFI

C-compatible struct layouts:

```aether
import std.ffi.ffi

func ffi_example() -> Int {
    // Create C-compatible struct
    let s = struct_new()
    struct_add_field(s, 4)  // int32
    struct_add_field(s, 8)  // int64
    
    let size = struct_size(s)  // 12 bytes (with padding)
    
    let instance = struct_alloc(s)
    struct_write_field(instance, 0, 12345)
    
    struct_read_field(instance, 0)  // 12345
}
```

### 6. Hot-Reloading

Live code updates (Erlang-style):

```aether
import std.hotreload.hotreload

func hot_reload_example() -> Int {
    let table = version_table_new()
    
    // Add initial version
    let v1 = version_add(table, code_v1, 100)
    
    // Update to new version
    let v2 = version_add(table, code_v2, 150)
    
    // Get active version
    version_get_active(table)
}
```

### 7. Query-Based Build

Incremental compilation:

```aether
import std.build.query

func incremental_build() -> Int {
    let cache = query_cache_new()
    
    query_set(cache, file1_hash, file1_result)
    query_set(cache, file2_hash, file2_result)
    
    // Later: check if rebuild needed
    if query_is_dirty(cache, file1_hash, new_hash) {
        // Rebuild file1
    }
    
    0
}
```

### 8. Link-Time Pruning

Dead code elimination:

```aether
import std.linker.prune

func optimized_binary() -> Int {
    let symbols = symbol_table_new()
    
    symbol_add(symbols, "main", 1)
    symbol_add(symbols, "helper", 1)
    symbol_add(symbols, "unused", 0)
    
    // Prune unused symbols
    let pruned = prune_unused(symbols)
    
    symbol_table_size(pruned)
}
```

### 9. Morphic Runtime

Bytecode VM with adaptive optimization:

```aether
import std.morphic.ir

func vm_example() -> Int {
    let vm = vm_new()
    
    // Build program
    let code = ae_malloc(40)
    ae_store64(code, encode_instr(OP_LOAD_IMM, 0, 0, 0, 10))
    ae_store64(code + 8, encode_instr(OP_LOAD_IMM, 1, 0, 0, 32))
    ae_store64(code + 16, encode_instr(OP_ADD, 2, 0, 1, 0))
    ae_store64(code + 24, encode_instr(OP_HALT, 0, 0, 0, 0))
    
    vm_load_code(vm, code, 4)
    vm_run(vm)
    
    vm_get_reg(vm, 2)  // 42
}
```

---

## Native Compilation

### Target Platforms

| Target | Format | Command |
|--------|--------|---------|
| macOS ARM64 | Mach-O | `--target macos-arm64` |
| macOS x64 | Mach-O | `--target macos-x64` |
| Linux x64 | ELF | `--target linux-x64` |
| Linux ARM64 | ELF | `--target linux-arm64` |

### Compilation Flags

```bash
# Basic compilation
./bootstrap/aetherc_native program.aether

# Specify output file
./bootstrap/aetherc_native program.aether -o myprogram

# Cross-compile for Linux
./bootstrap/aetherc_native program.aether --target linux-x64 -o server
```

### Binary Internals

The compiler generates:
1. **ARM64/x86_64 machine code** directly
2. **Proper executable headers** (Mach-O or ELF)
3. **No runtime dependencies**

---

## Standard Library

### Core Modules (87 files)

| Category | Modules |
|----------|---------|
| **Core** | std, prelude, lib, collections |
| **I/O** | std.io, std.io.fs, std.net |
| **Runtime** | vec, map, str, arena, alloc, async, parallel |
| **Types** | bounded, liquid |
| **Time** | time, datetime |
| **Crypto** | crypto |
| **Web** | frontend, backend, ui |
| **Cloud** | cloud (AWS, GCP, Azure) |
| **Database** | postgres, firebase |
| **Compiler** | lexer, parser, typechecker, codegen |
| **Native** | arm64, x86_64, elf, macho, pe |
| **Veritas** | comptime, dependent, actors, effects, ffi, hotreload |

### Importing Everything

```aether
import lib  // Imports all 85 modules
```

---

## Best Practices

### 1. Use Meaningful Names

```aether
// Good
func calculate_area(width: Int, height: Int) -> Int {
    width * height
}

// Bad
func f(w: Int, h: Int) -> Int {
    w * h
}
```

### 2. Keep Functions Small

```aether
// Good: Each function does one thing
func validate_input(x: Int) -> Int {
    if x < 0 { return 0 }
    if x > 1000 { return 0 }
    1
}

func process(x: Int) -> Int {
    x * 2
}

func main() -> Int {
    let x = 50
    if validate_input(x) == 1 {
        return process(x)
    }
    0
}
```

### 3. Use Constants for Magic Numbers

```aether
const MAX_SIZE: Int = 1024
const ERROR_CODE: Int = 0 - 1

func allocate() -> Int {
    ae_malloc(MAX_SIZE)
}
```

### 4. Handle Errors Explicitly

```aether
func safe_operation(x: Int) -> Int {
    if x == 0 {
        return ERROR_CODE
    }
    100 / x
}

func main() -> Int {
    let result = safe_operation(0)
    if result == ERROR_CODE {
        return 1  // Handle error
    }
    result
}
```

### 5. Comment Complex Logic

```aether
// Calculate the nth Fibonacci number using iterative approach
// Time: O(n), Space: O(1)
func fibonacci(n: Int) -> Int {
    if n <= 1 { return n }
    
    let prev = 0
    let curr = 1
    let i = 2
    
    while i <= n {
        let next = prev + curr
        prev = curr
        curr = next
        i = i + 1
    }
    
    curr
}
```

---

## Examples

### Example 1: FizzBuzz

```aether
func fizzbuzz(n: Int) -> Int {
    let i = 1
    let count = 0
    
    while i <= n {
        if i % 15 == 0 {
            count = count + 3  // FizzBuzz
        } else {
            if i % 3 == 0 {
                count = count + 1  // Fizz
            } else {
                if i % 5 == 0 {
                    count = count + 2  // Buzz
                }
            }
        }
        i = i + 1
    }
    
    count
}

func main() -> Int {
    fizzbuzz(100)
}
```

### Example 2: Binary Search

```aether
func binary_search(arr: Int, len: Int, target: Int) -> Int {
    let left = 0
    let right = len - 1
    
    while left <= right {
        let mid = left + (right - left) / 2
        let val = ae_load64(arr + mid * 8)
        
        if val == target {
            return mid
        }
        
        if val < target {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }
    
    0 - 1  // Not found
}
```

### Example 3: Quick Sort

```aether
func partition(arr: Int, low: Int, high: Int) -> Int {
    let pivot = ae_load64(arr + high * 8)
    let i = low - 1
    let j = low
    
    while j < high {
        if ae_load64(arr + j * 8) < pivot {
            i = i + 1
            // Swap
            let temp = ae_load64(arr + i * 8)
            ae_store64(arr + i * 8, ae_load64(arr + j * 8))
            ae_store64(arr + j * 8, temp)
        }
        j = j + 1
    }
    
    // Swap pivot
    let temp = ae_load64(arr + (i + 1) * 8)
    ae_store64(arr + (i + 1) * 8, ae_load64(arr + high * 8))
    ae_store64(arr + high * 8, temp)
    
    i + 1
}

func quicksort(arr: Int, low: Int, high: Int) -> Int {
    if low < high {
        let pi = partition(arr, low, high)
        quicksort(arr, low, pi - 1)
        quicksort(arr, pi + 1, high)
    }
    0
}
```

### Example 4: Hash Map

```aether
func hash(key: Int) -> Int {
    // FNV-1a hash
    let hash = 14695981039346656037
    let prime = 1099511628211
    
    hash = (hash * prime) + key
    hash
}

func map_new(capacity: Int) -> Int {
    let m = ae_malloc(16 + capacity * 16)
    ae_store64(m, capacity)
    ae_store64(m + 8, 0)  // size
    m
}

func map_put(m: Int, key: Int, value: Int) -> Int {
    let cap = ae_load64(m)
    let idx = hash(key) % cap
    let slot = m + 16 + idx * 16
    
    ae_store64(slot, key)
    ae_store64(slot + 8, value)
    
    let size = ae_load64(m + 8)
    ae_store64(m + 8, size + 1)
    0
}

func map_get(m: Int, key: Int) -> Int {
    let cap = ae_load64(m)
    let idx = hash(key) % cap
    let slot = m + 16 + idx * 16
    
    ae_load64(slot + 8)
}
```

### Example 5: Linked List

```aether
// Node: [value, next]
func node_new(value: Int) -> Int {
    let n = ae_malloc(16)
    ae_store64(n, value)
    ae_store64(n + 8, 0)  // next = null
    n
}

func list_push(head: Int, value: Int) -> Int {
    let new_node = node_new(value)
    ae_store64(new_node + 8, head)
    new_node
}

func list_sum(head: Int) -> Int {
    let sum = 0
    let curr = head
    
    while curr != 0 {
        sum = sum + ae_load64(curr)
        curr = ae_load64(curr + 8)
    }
    
    sum
}

func main() -> Int {
    let list = 0
    list = list_push(list, 10)
    list = list_push(list, 20)
    list = list_push(list, 30)
    
    list_sum(list)  // 60
}
```

---

## Contributing

1. All code must be pure Aether (no C dependencies)
2. Follow the style guide above
3. Write tests for new features
4. Update documentation

---

## License

MIT License - See LICENSE file

---

**Aether** - *The Future of Programming*

Built with ❤️ in Pure Aether
