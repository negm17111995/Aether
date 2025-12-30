# Compilation in Aether

## The Aether Compiler

The Aether compiler (`aetherc_native`) is a **100% self-hosted** native compiler.

### Key Properties

- Written in Aether (compiles itself)
- Zero C dependencies
- Direct machine code generation
- Cross-platform targeting

## Compilation Process

### 1. Lexical Analysis

Source code → Tokens

```
func main() → [ FUNC, IDENT(main), LPAREN, RPAREN, ARROW, ... ]
```

### 2. Parsing

Tokens → Abstract Syntax Tree (AST)

```
FunctionDecl {
    name: "main",
    params: [],
    return_type: Int,
    body: Block { ... }
}
```

### 3. Type Checking

AST → Typed AST

- Verify types match
- Check function signatures
- Validate operations

### 4. Code Generation

Typed AST → Machine Code

- ARM64 for Apple Silicon / ARM Linux
- x86_64 for Intel/AMD

### 5. Binary Generation

Machine Code → Executable

- Mach-O for macOS
- ELF for Linux
- PE for Windows

## Compiler Usage

### Basic Compilation

```bash
./bootstrap/aetherc_native program.aether
./a.out
```

### Specify Output

```bash
./bootstrap/aetherc_native program.aether -o myprogram
./myprogram
```

### Target Platforms

```bash
# macOS ARM64 (Apple Silicon)
./bootstrap/aetherc_native program.aether --target macos-arm64

# macOS x64 (Intel)
./bootstrap/aetherc_native program.aether --target macos-x64

# Linux x64
./bootstrap/aetherc_native program.aether --target linux-x64

# Linux ARM64
./bootstrap/aetherc_native program.aether --target linux-arm64
```

## Cross-Compilation

Compile for a different platform than your host:

```bash
# On macOS, compile for Linux
./bootstrap/aetherc_native server.aether --target linux-x64 -o server

# Transfer to Linux server
scp server user@server:/path/to/app
```

## Binary Formats

### Mach-O (macOS)

```
Mach-O 64-bit executable arm64
├── Header
├── Load Commands
│   ├── LC_SEGMENT_64 (__PAGEZERO)
│   ├── LC_SEGMENT_64 (__TEXT)
│   └── LC_UNIXTHREAD
└── __TEXT segment
    └── Machine code
```

### ELF (Linux)

```
ELF 64-bit LSB executable, x86-64
├── ELF Header
├── Program Header (PT_LOAD)
└── Code segment
```

## Code Generation Details

### ARM64 Calling Convention

- x0-x7: Arguments and return value
- x29: Frame pointer
- x30: Link register (return address)
- sp: Stack pointer

### x86_64 Calling Convention

- rdi, rsi, rdx, rcx, r8, r9: Arguments
- rax: Return value
- rbp: Frame pointer
- rsp: Stack pointer

### Register Allocation

The compiler:
1. Uses x0 for expression evaluation
2. Spills to stack when needed
3. Preserves callee-saved registers

## Optimization

### Current Optimizations

- Constant folding
- Dead code elimination (via Veritas)
- Register allocation

### Future Optimizations

- Inlining
- Loop unrolling
- SIMD vectorization

## Debugging

### Verify Compilation

```bash
# Check binary format
file a.out

# Expected outputs:
# Mach-O 64-bit executable arm64
# ELF 64-bit LSB executable, x86-64
```

### Check Exit Code

```bash
./a.out
echo $?
```

## Self-Hosting

The Aether compiler compiles itself:

```bash
# The compiler source is in stdlib/aether_compiler/
# Main files:
# - lexer.aether
# - parser.aether
# - typechecker.aether
# - native/codegen.aether
# - native/arm64.aether
# - native/x86_64.aether
# - native/macho.aether
# - native/elf.aether
```

### Bootstrapping

1. Initial bootstrap compiler in binary form
2. Compiles Aether source → new compiler
3. New compiler can compile itself
4. True self-hosting achieved
