# Getting Started with Aether

## What is Aether?

Aether is a revolutionary programming language that breaks all traditional boundaries:

- **100% Self-Hosted**: The compiler is written in Aether and compiles itself
- **Zero Dependencies**: No C, no libc, no runtime libraries
- **Native Performance**: Compiles directly to machine code (ARM64/x86_64)
- **Universal**: One language for systems, web, cloud, AI, and everything

## Philosophy

Aether is designed around three core principles:

### 1. Simplicity Without Sacrifice
Every feature must justify its existence. If it adds complexity without proportional value, it doesn't belong.

### 2. Performance is Non-Negotiable
Aether generates machine code that rivals hand-written assembly. No garbage collector pauses, no JIT warmup time.

### 3. Safety Through Design
Memory safety, type safety, and concurrency safety are built into the language's DNA, not bolted on as an afterthought.

## Installation

### Prerequisites
- macOS (Apple Silicon or Intel) or Linux (x64 or ARM64)
- Terminal access
- Git (for cloning)

### Step 1: Clone the Repository
```bash
git clone https://github.com/your-repo/aether-core.git
cd aether-core
```

### Step 2: Verify the Compiler
```bash
./bootstrap/aetherc_native --help
```

Output:
```
Aether Native Compiler 8.0 (World-Class)
Usage: aetherc <file.aether>
```

### Step 3: Create Your First Program
```bash
echo 'func main() -> Int { 42 }' > hello.aether
./bootstrap/aetherc_native hello.aether
./a.out
echo "Exit code: $?"
```

Expected output: `Exit code: 42`

## Your First Real Program

### Hello World (Exit Code Style)
```aether
// hello.aether
// Aether expresses output via exit codes
// This program exits with code 42

func main() -> Int {
    42
}
```

### A More Complex Example
```aether
// calculator.aether
// Calculate the sum of 1 to 100

func sum_to_n(n: Int) -> Int {
    let sum = 0
    let i = 1
    while i <= n {
        sum = sum + i
        i = i + 1
    }
    sum
}

func main() -> Int {
    sum_to_n(100)  // Returns 5050
}
```

## Compilation Targets

### macOS ARM64 (Apple Silicon)
```bash
./bootstrap/aetherc_native program.aether --target macos-arm64
```

### macOS x64 (Intel)
```bash
./bootstrap/aetherc_native program.aether --target macos-x64
```

### Linux x64
```bash
./bootstrap/aetherc_native program.aether --target linux-x64 -o server
```

### Linux ARM64
```bash
./bootstrap/aetherc_native program.aether --target linux-arm64
```

## Project Structure

```
aether-core/
├── bootstrap/
│   └── aetherc_native    # The compiler binary
├── stdlib/
│   ├── std.aether        # Core standard library
│   ├── lib.aether        # Module index (87 modules)
│   ├── prelude.aether    # Auto-imported basics
│   ├── std/              # Standard library modules
│   │   ├── io.aether
│   │   ├── net.aether
│   │   ├── collections.aether
│   │   ├── runtime/      # Runtime support
│   │   ├── crypto/       # Cryptography
│   │   ├── web/          # Web development
│   │   ├── cloud/        # Cloud services
│   │   └── ...
│   └── aether_compiler/  # Compiler source (in Aether!)
│       ├── lexer.aether
│       ├── parser.aether
│       ├── typechecker.aether
│       ├── native/       # Native codegen
│       └── veritas/      # Advanced features
└── README.md
```

## Next Steps

1. Read the [Language Basics](02_LANGUAGE_BASICS.md)
2. Understand [Types and Variables](03_TYPES_AND_VARIABLES.md)
3. Master [Control Flow](04_CONTROL_FLOW.md)
4. Learn [Functions](05_FUNCTIONS.md)
5. Explore [Memory Management](06_MEMORY.md)
