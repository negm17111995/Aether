# PART 1: INTRODUCTION TO AETHER

## 1.1 What is Aether?

Aether is a **systems programming language** built from the ground up for:

- **Maximum Performance**: Compiles to native machine code with zero runtime overhead
- **Memory Safety**: Ownership system prevents memory bugs at compile time
- **Simplicity**: Clean, minimal syntax inspired by Rust, Go, and Swift
- **Self-Hosting**: The entire compiler is written in Aether itself

### 1.1.1 The Philosophy

Aether follows three core principles:

1. **Zero-Cost Abstractions**: High-level features compile to the same code you would write by hand
2. **Correctness by Default**: The type system catches bugs before they run
3. **Explicit Over Implicit**: No hidden allocations, no hidden control flow

### 1.1.2 How Aether Compares

| Feature | Aether | C | C++ | Rust | Go |
|---------|--------|---|-----|------|-----|
| Memory Safety | ✅ Compile-time | ❌ Manual | ❌ Manual | ✅ Compile-time | ✅ GC |
| Garbage Collector | ❌ None | ❌ None | ❌ None | ❌ None | ✅ Yes |
| Null Pointers | ❌ None | ✅ Everywhere | ✅ Everywhere | ❌ None | ✅ Nil |
| Data Races | ❌ Impossible | ✅ Possible | ✅ Possible | ❌ Impossible | ✅ Possible |
| Generics | ✅ Yes | ❌ Macros | ✅ Templates | ✅ Yes | ✅ Yes |
| Binary Size | Tiny | Small | Medium | Large | Large |
| Compile Speed | Fast | Fast | Slow | Slow | Fast |

---

## 1.2 Installation

### 1.2.1 Prerequisites

- macOS 11+ (Apple Silicon or Intel)
- Linux (x86_64 or ARM64)
- 4GB RAM minimum
- 1GB disk space

### 1.2.2 Installation Steps

```bash
# Clone the repository
git clone https://github.com/aether-lang/aether-core.git
cd aether-core

# Verify the compiler
./aetherc --version
# Output: Aether Compiler v1.0.0
```

### 1.2.3 Directory Structure

```
aether-core/
├── aetherc              # The compiler executable
├── stdlib/              # Standard library (86 modules)
│   ├── core/           # Core types (Result, Option, etc.)
│   ├── runtime/        # Runtime support
│   ├── compiler/       # Self-hosted compiler
│   ├── collections/    # Data structures
│   ├── io/            # Input/Output
│   ├── net/           # Networking
│   └── tools/         # REPL, formatter
├── docs/               # Documentation
└── examples/           # Example programs
```

---

## 1.3 Your First Program

### 1.3.1 Hello World

Create a file named `hello.aether`:

```aether
// This is a comment
// Every Aether program starts with a main function

func main() -> Int {
    // Print to console
    print("Hello, World!")
    
    // Return 0 to indicate success
    // (like C's exit code)
    0
}
```

**Line-by-line explanation:**

1. `// This is a comment` - Single-line comments start with `//`
2. `func main() -> Int` - Define the main function that returns an integer
3. `{` - Open the function body
4. `print("Hello, World!")` - Call the built-in print function
5. `0` - The last expression is the return value (no `return` needed)
6. `}` - Close the function body

### 1.3.2 Compiling and Running

```bash
# Compile the program
./aetherc hello.aether

# This creates an executable named 'a.out'
# Run it:
./a.out

# Output:
# Hello, World!
```

### 1.3.3 Compiler Options

| Option | Description | Example |
|--------|-------------|---------|
| `-o name` | Set output filename | `./aetherc -o hello hello.aether` |
| `-O0` | No optimization | `./aetherc -O0 prog.aether` |
| `-O1` | Basic optimization | `./aetherc -O1 prog.aether` |
| `-O2` | Standard optimization | `./aetherc -O2 prog.aether` |
| `-O3` | Maximum optimization | `./aetherc -O3 prog.aether` |
| `-S` | Output assembly only | `./aetherc -S prog.aether` |
| `--version` | Print version | `./aetherc --version` |

---

## 1.4 Program Structure

### 1.4.1 Basic Structure

Every Aether program consists of:

```aether
// 1. IMPORTS (optional)
use std::io
use std::collections

// 2. CONSTANTS (optional)
const MAX_SIZE: Int = 1024
const VERSION: String = "1.0.0"

// 3. TYPE DEFINITIONS (optional)
struct Point {
    x: Int,
    y: Int,
}

enum Color {
    Red,
    Green,
    Blue,
}

// 4. FUNCTIONS
func helper() -> Int {
    42
}

// 5. MAIN FUNCTION (required for executables)
func main() -> Int {
    // Program starts here
    0
}
```

### 1.4.2 Execution Flow

1. Program starts at `main()`
2. Statements execute top to bottom
3. Function calls transfer control
4. `main()` returns exit code (0 = success)

---

## 1.5 Comments

### 1.5.1 Single-Line Comments

```aether
// This is a single-line comment
let x = 5  // Comment at end of line
```

### 1.5.2 Multi-Line Comments

```aether
/*
   This is a multi-line comment.
   It can span multiple lines.
   Useful for longer explanations.
*/
```

### 1.5.3 Documentation Comments

```aether
/// This is a documentation comment.
/// It documents the item that follows.
/// 
/// # Examples
/// ```
/// let x = add(2, 3)
/// assert(x == 5)
/// ```
func add(a: Int, b: Int) -> Int {
    a + b
}
```

---

## 1.6 Keywords

Aether has these reserved keywords:

| Category | Keywords |
|----------|----------|
| **Declarations** | `func`, `let`, `const`, `struct`, `enum`, `trait`, `impl`, `type` |
| **Control Flow** | `if`, `else`, `match`, `while`, `for`, `loop`, `break`, `continue`, `return` |
| **Values** | `true`, `false`, `self`, `Self` |
| **Modifiers** | `mut`, `pub`, `use`, `as`, `in` |
| **Memory** | `unsafe` |
| **Async** | `async`, `await`, `spawn` |

---

*Next: [Part 2: Variables and Types](02_VARIABLES_AND_TYPES.md)*
