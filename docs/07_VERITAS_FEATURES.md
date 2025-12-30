# Veritas Features - Advanced Aether

The Veritas system provides advanced compile-time and runtime features.

## 1. Hardware-Aware Comptime

Detect and optimize for specific hardware at compile time.

### CPU Detection

```aether
import std.comptime.hw

func main() -> Int {
    let cpu = detect_cpu_arch()
    
    // Returns:
    // 0 = Unknown
    // 1 = x86_64
    // 2 = ARM64
    // 3 = Apple M1
    // 4 = Apple M2
    // 5 = Apple M3
    // 6 = Apple M4
    
    cpu
}
```

### SIMD Detection

```aether
func check_simd() -> Int {
    let simd = detect_simd_capability()
    
    // Returns:
    // 0 = None
    // 1 = SSE2
    // 2 = AVX
    // 3 = AVX2
    // 4 = AVX512
    // 10 = ARM NEON
    
    simd
}
```

### Optimized Code Paths

```aether
func fast_multiply(a: Int, b: Int) -> Int {
    let simd = detect_simd_capability()
    
    if simd == SIMD_NEON {
        return neon_multiply(a, b)
    }
    if simd == SIMD_AVX2 {
        return avx2_multiply(a, b)
    }
    
    a * b   // Fallback
}
```

## 2. Dependent Types

Types that depend on runtime values.

### Bounded Arrays

```aether
import aether_compiler.veritas.dependent

func bounded_example() -> Int {
    // Create array with max capacity 10
    let arr = bounded_new(10)
    
    // Push elements (validated at runtime)
    bounded_push(arr, 100)  // OK
    bounded_push(arr, 200)  // OK
    
    // Get element
    let val = bounded_get(arr, 0)   // 100
    
    // Length check
    let len = bounded_len(arr)      // 2
    
    val
}
```

### Capacity Enforcement

```aether
func test_bounds() -> Int {
    let arr = bounded_new(2)    // Max 2 elements
    
    bounded_push(arr, 1)        // OK
    bounded_push(arr, 2)        // OK
    
    let result = bounded_push(arr, 3)  // Returns 0 (rejected!)
    
    result  // 0
}
```

## 3. Actor Model

Message-passing concurrency with isolated state.

### Creating Actors

```aether
import std.actor.actor

func actor_example() -> Int {
    // Spawn a new actor
    let worker = actor_spawn()
    
    // Send message
    actor_send(worker, 42)
    
    // Receive response
    let result = actor_recv(worker)
    
    result
}
```

### Actor Communication Pattern

```aether
func parallel_compute() -> Int {
    let a1 = actor_spawn()
    let a2 = actor_spawn()
    
    // Send work
    actor_send(a1, 100)
    actor_send(a2, 200)
    
    // Collect results
    let r1 = actor_recv(a1)
    let r2 = actor_recv(a2)
    
    r1 + r2
}
```

## 4. Algebraic Effects

Structured handling of side effects.

### Effect Handling

```aether
import std.effects.effects

func divide(a: Int, b: Int) -> Int {
    if b == 0 {
        effect_raise(ERROR_DIVIDE_BY_ZERO)
        return 999  // Error marker
    }
    a / b
}

func safe_compute() -> Int {
    let handler = effect_handler_new()
    effect_push(handler)
    
    let result = divide(100, 0)
    
    effect_pop()
    result  // 999 (error case)
}
```

### Effect Types

```aether
const EFFECT_NONE: Int = 0
const EFFECT_ERROR: Int = 1
const EFFECT_IO: Int = 2
const EFFECT_STATE: Int = 3
```

## 5. Bit-Level FFI

C-compatible struct layouts for foreign function interface.

### Struct Definition

```aether
import std.ffi.ffi

func define_struct() -> Int {
    let s = struct_new()
    
    // Add fields (size in bytes)
    struct_add_field(s, 4)  // int32_t
    struct_add_field(s, 8)  // int64_t
    
    struct_size(s)  // 12 bytes (with alignment)
}
```

### Struct Usage

```aether
func use_struct() -> Int {
    let s = struct_new()
    struct_add_field(s, 4)
    struct_add_field(s, 8)
    
    // Allocate instance
    let obj = struct_alloc(s)
    
    // Write field 0
    struct_write_field(obj, 0, 12345)
    
    // Read field 0
    let val = struct_read_field(obj, 0)
    
    val  // 12345
}
```

## 6. Hot-Reloading

Update running code without restart (Erlang-style).

### Version Management

```aether
import std.hotreload.hotreload

func versioned_code() -> Int {
    // Create version table
    let table = version_table_new()
    
    // Add version 1
    let v1 = version_add(table, code_ptr_v1, 100)
    
    // Later: Add version 2
    let v2 = version_add(table, code_ptr_v2, 150)
    
    // Get active (latest) version
    let active = version_get_active(table)
    
    active  // Points to v2 code
}
```

## 7. Query-Based Build

Incremental compilation with automatic dependency tracking.

### Build Cache

```aether
import std.build.query

func incremental_build() -> Int {
    // Create cache
    let cache = query_cache_new()
    
    // Store compilation results
    let hash1 = compute_hash(file1)
    query_set(cache, hash1, result1)
    
    // Later: Check if needs rebuild
    let new_hash = compute_hash(file1)
    let dirty = query_is_dirty(cache, hash1, new_hash)
    
    if dirty {
        // Recompile
    }
    
    dirty
}
```

## 8. Link-Time Pruning

Remove unused code from final binary.

### Symbol Table

```aether
import std.linker.prune

func optimize_binary() -> Int {
    let symbols = symbol_table_new()
    
    // Mark used symbols
    symbol_add(symbols, "main", 1)
    symbol_add(symbols, "helper", 1)
    symbol_add(symbols, "unused", 0)
    
    // Prune unused
    let pruned = prune_unused(symbols)
    
    // Get final size
    symbol_table_size(pruned)
}
```

## 9. Morphic Runtime

Bytecode VM with adaptive optimization.

### VM Creation

```aether
import std.morphic.ir

func vm_example() -> Int {
    let vm = vm_new()
    
    // Build bytecode
    let code = ae_malloc(32)
    
    // LOAD_IMM R0, 10
    ae_store64(code, encode_instr(OP_LOAD_IMM, 0, 0, 0, 10))
    
    // LOAD_IMM R1, 32
    ae_store64(code + 8, encode_instr(OP_LOAD_IMM, 1, 0, 0, 32))
    
    // ADD R2, R0, R1
    ae_store64(code + 16, encode_instr(OP_ADD, 2, 0, 1, 0))
    
    // HALT
    ae_store64(code + 24, encode_instr(OP_HALT, 0, 0, 0, 0))
    
    // Load and run
    vm_load_code(vm, code, 4)
    vm_run(vm)
    
    // Get result from R2
    vm_get_reg(vm, 2)  // 42
}
```

### Opcodes

| Opcode | Description |
|--------|-------------|
| OP_NOP | No operation |
| OP_LOAD_IMM | Load immediate value |
| OP_LOAD_REG | Copy register |
| OP_ADD | Add registers |
| OP_SUB | Subtract |
| OP_MUL | Multiply |
| OP_DIV | Divide |
| OP_JMP | Jump |
| OP_JZ | Jump if zero |
| OP_CALL | Call function |
| OP_RET | Return |
| OP_HALT | Stop execution |
