# Aether Programming Language

> **The World's First Truly Universal Programming Language**
> Self-hosted • Zero Dependencies • Native Performance • Cross-Platform

---

## Quick Start

```bash
# Clone
git clone https://github.com/your-repo/aether-core.git
cd aether-core

# Compile
echo 'func main() -> Int { 42 }' > hello.aether
./bootstrap/aetherc_native hello.aether
./a.out
# Exit code: 42
```

## Features

| Feature | Description |
|---------|-------------|
| **Self-Hosted** | Compiler written in Aether |
| **Zero C** | No C dependencies whatsoever |
| **Native** | Compiles to ARM64/x86_64 machine code |
| **Cross-Platform** | macOS, Linux (Windows coming) |
| **87 Modules** | Complete standard library |
| **Veritas** | 9 advanced features |

## Documentation

### Core Documentation

| Doc | Description |
|-----|-------------|
| [Getting Started](docs/01_GETTING_STARTED.md) | Installation, first program, project structure |
| [Language Basics](docs/02_LANGUAGE_BASICS.md) | Syntax, expressions, statements |
| [Types & Variables](docs/03_TYPES_AND_VARIABLES.md) | Int, Bool, let, const |
| [Control Flow](docs/04_CONTROL_FLOW.md) | if/else, while loops |
| [Functions](docs/05_FUNCTIONS.md) | Declaration, parameters, recursion |
| [Memory](docs/06_MEMORY.md) | Stack, heap, ae_malloc, data structures |
| [Veritas Features](docs/07_VERITAS_FEATURES.md) | 9 advanced features |
| [Standard Library](docs/08_STANDARD_LIBRARY.md) | 87 modules overview |
| [Examples](docs/09_EXAMPLES.md) | 15+ complete examples |
| [Compilation](docs/10_COMPILATION.md) | Compiler internals, cross-compilation |

## Language Overview

### Hello World
```aether
func main() -> Int {
    0  // Exit code 0 = success
}
```

### Variables
```aether
let x = 42
let y: Int = 100
x = x + y
```

### Functions
```aether
func add(a: Int, b: Int) -> Int {
    a + b
}

func main() -> Int {
    add(20, 22)  // 42
}
```

### Control Flow
```aether
func abs(x: Int) -> Int {
    if x < 0 {
        return 0 - x
    }
    x
}

func sum_to(n: Int) -> Int {
    let sum = 0
    let i = 1
    while i <= n {
        sum = sum + i
        i = i + 1
    }
    sum
}
```

### Memory
```aether
func example() -> Int {
    let ptr = ae_malloc(1024)
    ae_store64(ptr, 42)
    ae_load64(ptr)
}
```

## Targets

| Target | Command |
|--------|---------|
| macOS ARM64 | `--target macos-arm64` |
| macOS x64 | `--target macos-x64` |
| Linux x64 | `--target linux-x64` |
| Linux ARM64 | `--target linux-arm64` |

## Statistics

| Metric | Value |
|--------|-------|
| Aether Files | 87 |
| C Files | 0 |
| Standard Library Modules | 87 |
| Documentation Lines | 2,830+ |
| Veritas Features | 9 |

## License

MIT License

---

**Aether** - *The Future of Programming*

Built with ❤️ in Pure Aether
