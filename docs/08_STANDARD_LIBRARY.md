# Aether Standard Library

## Library Overview

The Aether standard library contains **87 modules** organized by category.

## Importing Modules

```aether
import std                  // Core standard library
import std.io               // I/O operations
import std.collections      // Data structures
import lib                  // All 87 modules at once
```

## Core Modules

### std.aether

Core primitives and utilities.

```aether
import std

// Memory operations
let ptr = ae_malloc(1024)
ae_store64(ptr, 42)
let val = ae_load64(ptr)
ae_store8(ptr, 255)
let byte = ae_load8(ptr)

// Math utilities
let abs_val = ae_abs(0 - 42)    // 42
let min_val = ae_min(10, 20)     // 10
let max_val = ae_max(10, 20)     // 20
let clamped = ae_clamp(50, 0, 100)  // 50
```

### std.collections

Dynamic data structures.

```aether
import std.collections

// Vector
let v = vec_new()
vec_push(v, 10)
vec_push(v, 20)
vec_push(v, 30)
let len = vec_len(v)        // 3
let item = vec_get(v, 1)    // 20
vec_set(v, 0, 100)
```

## I/O Modules

### std.io

Basic I/O operations.

```aether
import std.io

// File operations concepts
// (Low-level syscall wrappers)
```

### std.io.fs

File system operations.

```aether
import std.io.fs

// File operations via syscalls
```

### std.net

Networking primitives.

```aether
import std.net

// Socket operations
// TCP/UDP networking
```

## Runtime Modules

### std.runtime.vec

Vector implementation.

```aether
import std.runtime.vec

let v = vec_new()
vec_push(v, 42)
let x = vec_get(v, 0)
let len = vec_len(v)
```

### std.runtime.map

Hash map implementation.

```aether
import std.runtime.map

let m = map_new()
map_put(m, key, value)
let val = map_get(m, key)
```

### std.runtime.str

String operations.

```aether
import std.runtime.str

let len = str_len(s)
let cmp = str_cmp(s1, s2)
```

### std.runtime.arena

Arena allocator for efficient memory management.

```aether
import std.runtime.arena

let arena = arena_new(1024 * 1024)  // 1MB
let ptr = arena_alloc(arena, 64)
arena_reset(arena)
```

### std.runtime.alloc

General allocator.

```aether
import std.runtime.alloc

let ptr = alloc(1024)
let ptr2 = alloc_zeroed(1024)
```

### std.runtime.async

Asynchronous operations.

```aether
import std.runtime.async

// Async patterns and futures
```

### std.runtime.parallel

Parallel execution.

```aether
import std.runtime.parallel

// Parallel map, reduce, etc.
```

### std.runtime.concurrency

Concurrency primitives.

```aether
import std.runtime.concurrency

// Locks, channels, etc.
```

### std.runtime.simd

SIMD operations for ARM.

```aether
import std.runtime.simd

// ARM NEON operations
```

### std.runtime.simd_x64

SIMD for x86_64.

```aether
import std.runtime.simd_x64

// SSE/AVX operations
```

## Time Modules

### std.time.time

Time operations.

```aether
import std.time.time

let now = time_now()
let elapsed = time_diff(start, end)
```

### std.time.datetime

Date and time handling.

```aether
import std.time.datetime

// Date parsing and formatting
```

## Crypto Modules

### std.crypto.crypto

Cryptographic operations.

```aether
import std.crypto.crypto

// Hashing, encryption
let hash = crypto_hash(data, len)
```

## Web Modules

### std.web.frontend

Frontend web development.

```aether
import std.web.frontend

// HTML, CSS, DOM operations
```

### std.web.backend

Backend web development.

```aether
import std.web.backend

// HTTP server, routing
```

### std.web.ui

UI components.

```aether
import std.web.ui

// UI component system
```

## Cloud Modules

### std.cloud.cloud

Cloud provider abstractions.

```aether
import std.cloud.cloud

// AWS, GCP, Azure abstractions
```

## Database Modules

### std.db.anc_decoder

Database operations.

```aether
import std.db.anc_decoder

// Query building, result parsing
```

## Compiler Modules

### aether_compiler.lexer

Lexical analysis.

```aether
import aether_compiler.lexer

let lexer = lexer_new(source)
let token = lexer_next(lexer)
```

### aether_compiler.parser

Parsing.

```aether
import aether_compiler.parser

let parser = parser_new(tokens)
let ast = parser_parse(parser)
```

### aether_compiler.typechecker

Type checking.

```aether
import aether_compiler.typechecker

// Type inference and checking
```

### aether_compiler.native.codegen

Native code generation.

```aether
import aether_compiler.native.codegen

// ARM64/x86_64 code generation
```

## Module Summary

| Category | Modules |
|----------|---------|
| Core | std, prelude, lib, collections |
| I/O | io, io.fs, net |
| Runtime | vec, map, str, arena, alloc, async, parallel, concurrency, simd |
| Types | bounded, liquid |
| Time | time, datetime |
| Crypto | crypto |
| Web | frontend, backend, ui |
| Cloud | cloud |
| Database | anc_decoder |
| Compiler | lexer, parser, typechecker, codegen |
| Native | arm64, x86_64, elf, macho, pe |
| Veritas | comptime, dependent, actors, effects, ffi, hotreload, prune, query |
