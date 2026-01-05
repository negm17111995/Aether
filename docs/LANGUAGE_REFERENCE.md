# Aether Language Reference

## Overview

Aether is a systems programming language designed for:
- **Performance**: Compiles to native machine code
- **Safety**: Strong type system with memory safety
- **Simplicity**: Clean, minimal syntax
- **Self-hosting**: The compiler is written in Aether

## Basic Syntax

### Functions

```aether
func add(a: Int, b: Int) -> Int {
    a + b
}

func main(argc: Int, argv: Int) -> Int {
    let result = add(1, 2)
    0
}
```

### Variables

```aether
let x = 42              // Immutable
let mut y = 10          // Mutable
y = y + 1
```

### Control Flow

```aether
// If statement
if x > 0 {
    print(43)  // +
} else {
    print(45)  // -
}

// While loop
let i = 0
while i < 10 {
    print(i + 48)  // Print digit
    i = i + 1
}
```

### Constants

```aether
const PI: Int = 3
const MAX_SIZE: Int = 1024
```

### Structs

```aether
struct Point {
    x: Int,
    y: Int,
}

func point_new(x: Int, y: Int) -> Int {
    let p = __builtin_malloc(16)
    __builtin_store64(p, x)
    __builtin_store64(p + 8, y)
    p
}
```

## Built-in Functions

### Memory

| Function | Description |
|----------|-------------|
| `__builtin_malloc(size)` | Allocate memory |
| `__builtin_load8(ptr)` | Load 8-bit value |
| `__builtin_load64(ptr)` | Load 64-bit value |
| `__builtin_store8(ptr, val)` | Store 8-bit value |
| `__builtin_store64(ptr, val)` | Store 64-bit value |

### I/O

| Function | Description |
|----------|-------------|
| `print(char)` | Print ASCII character |
| `__builtin_open(path, flags, mode)` | Open file |
| `__builtin_read(fd, buf, len)` | Read from file |
| `__builtin_write(fd, buf, len)` | Write to file |
| `__builtin_close(fd)` | Close file |

### Networking

| Function | Description |
|----------|-------------|
| `__builtin_socket(domain, type, protocol)` | Create socket |
| `__builtin_connect(fd, addr, len)` | Connect to server |
| `__builtin_bind(fd, addr, len)` | Bind to address |
| `__builtin_listen(fd, backlog)` | Listen for connections |
| `__builtin_accept(fd, addr, len)` | Accept connection |

## Compilation

```bash
# Compile a program
./aetherc_native program.aether -o program

# Run
./program
```

## Platform Support

- macOS ARM64 (Apple Silicon)
- macOS x64
- Linux ARM64
- Linux x64
