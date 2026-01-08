#!/usr/bin/env python3
"""
AETHER FUZZER - Find Bugs Before Users Do
Generates random/adversarial inputs to test the Aether compiler.

Targets:
1. Lexer: Random character sequences, edge-case tokens
2. Parser: Malformed ASTs, unbalanced brackets, deep nesting
3. Type Checker: Invalid type combinations
4. Codegen: Edge-case expressions

Usage:
    python3 fuzzer.py --target lexer --iterations 10000
    python3 fuzzer.py --target parser --iterations 10000
    python3 fuzzer.py --target all --iterations 1000
"""

import argparse
import os
import random
import string
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path
from typing import List, Optional, Tuple

# ============================================================================
# Configuration
# ============================================================================

AETHERC_PATH = "./bin/aetherc"
TIMEOUT_SECONDS = 5
MAX_FILE_SIZE = 10000
CRASH_DIR = "./crashes"

# ============================================================================
# Random Generators
# ============================================================================

def random_identifier(max_len: int = 20) -> str:
    """Generate a random identifier."""
    length = random.randint(1, max_len)
    first = random.choice(string.ascii_letters + "_")
    rest = ''.join(random.choices(string.ascii_letters + string.digits + "_", k=length-1))
    return first + rest

def random_number() -> str:
    """Generate a random number literal."""
    choice = random.randint(0, 3)
    if choice == 0:
        return str(random.randint(-2**63, 2**63 - 1))
    elif choice == 1:
        return str(random.random())
    elif choice == 2:
        return f"0x{random.randint(0, 2**64):x}"
    else:
        return f"0b{random.randint(0, 255):b}"

def random_string() -> str:
    """Generate a random string literal."""
    length = random.randint(0, 100)
    chars = ''.join(random.choices(string.printable.replace('"', '').replace('\\', ''), k=length))
    return f'"{chars}"'

def random_operator() -> str:
    """Generate a random operator."""
    ops = ["+", "-", "*", "/", "%", "==", "!=", "<", ">", "<=", ">=", 
           "&&", "||", "&", "|", "^", "<<", ">>", "=", "+=", "-="]
    return random.choice(ops)

def random_keyword() -> str:
    """Generate a random keyword."""
    keywords = ["func", "let", "const", "if", "else", "while", "for", "return",
                "struct", "import", "spawn", "match", "in", "mut", "pub"]
    return random.choice(keywords)

def random_type() -> str:
    """Generate a random type."""
    types = ["Int", "Float", "Bool", "Char", "String", random_identifier()]
    return random.choice(types)

# ============================================================================
# Fuzzing Strategies
# ============================================================================

@dataclass
class FuzzResult:
    """Result of a single fuzz test."""
    input_text: str
    exit_code: int
    stdout: str
    stderr: str
    timed_out: bool
    crashed: bool

def fuzz_lexer() -> str:
    """Generate input to stress the lexer."""
    strategies = [
        # Random character soup
        lambda: ''.join(random.choices(string.printable, k=random.randint(1, 1000))),
        
        # Very long identifiers
        lambda: "let " + "a" * random.randint(1000, 10000) + " = 1",
        
        # Very long numbers
        lambda: "let x = " + "9" * random.randint(100, 1000),
        
        # Mixed valid/invalid tokens
        lambda: ' '.join([random.choice([random_identifier(), random_operator(), 
                                          random_number(), random_keyword()]) 
                          for _ in range(random.randint(10, 100))]),
        
        # Unicode chaos
        lambda: ''.join(chr(random.randint(0, 0xFFFF)) for _ in range(100)),
        
        # Null bytes
        lambda: "let x = " + '\x00' * random.randint(1, 10) + "1",
        
        # Very long strings
        lambda: 'let s = "' + "x" * random.randint(1000, 10000) + '"',
        
        # Unterminated strings
        lambda: 'let s = "hello',
        
        # Nested comments
        lambda: "/* " * random.randint(10, 100) + " */",
        
        # Emoji and special chars
        lambda: "let 🚀 = 1 + 💀",
    ]
    
    return random.choice(strategies)()

def fuzz_parser() -> str:
    """Generate input to stress the parser."""
    strategies = [
        # Unbalanced braces
        lambda: "func main() {" * random.randint(10, 100),
        lambda: "}" * random.randint(10, 100),
        
        # Deep nesting
        lambda: "if 1 { " * random.randint(50, 200) + "let x = 1" + " }" * random.randint(50, 200),
        
        # Malformed function
        lambda: f"func {random_identifier()}(,,,) {{ }}",
        
        # Missing semicolons
        lambda: '\n'.join([f"let {random_identifier()} = {random_number()}" 
                           for _ in range(random.randint(5, 20))]),
        
        # Incomplete expressions
        lambda: f"let x = 1 + ",
        lambda: f"let x = (1 + 2",
        
        # Random keyword sequences
        lambda: ' '.join([random_keyword() for _ in range(random.randint(10, 50))]),
        
        # Empty constructs
        lambda: "func () {{ }}",
        lambda: "struct {{ }}",
        lambda: "if {{ }}",
        
        # Deeply nested expressions
        lambda: "let x = " + "(" * random.randint(100, 500) + "1" + ")" * random.randint(100, 500),
        
        # Many parameters
        lambda: "func f(" + ", ".join([f"a{i}: Int" for i in range(random.randint(100, 500))]) + ") {}",
        
        # Long chains
        lambda: "let x = " + ".field".join(["obj"] + [random_identifier() for _ in range(100)]),
    ]
    
    return random.choice(strategies)()

def fuzz_types() -> str:
    """Generate input to stress the type checker."""
    strategies = [
        # Type mismatch
        lambda: 'func main() { let x: Int = "hello" }',
        
        # Unknown types
        lambda: f'func main() {{ let x: {random_identifier()} = 1 }}',
        
        # Recursive types
        lambda: 'struct A { b: B } struct B { a: A }',
        
        # Self-referential
        lambda: 'struct S { s: S }',
        
        # Very long type names
        lambda: f'let x: {"A" * random.randint(100, 1000)} = 1',
        
        # Generic abuse
        lambda: f'let x: Vec<Vec<Vec<Vec<Vec<Vec<Vec<Vec<Int>>>>>>>> = 1',
        
        # Function type complexity
        lambda: 'func f(g: func(func(func(Int) -> Int) -> Int) -> Int) {}',
    ]
    
    return random.choice(strategies)()

def fuzz_codegen() -> str:
    """Generate input to stress code generation."""
    strategies = [
        # Large literal arrays
        lambda: "let arr = [" + ", ".join([str(random.randint(0, 1000)) 
                                            for _ in range(random.randint(100, 1000))]) + "]",
        
        # Extreme arithmetic
        lambda: "func main() { let x = " + " + ".join([random_number() 
                                                        for _ in range(random.randint(50, 200))]) + " }",
        
        # Many local variables
        lambda: "func main() { " + "\n".join([f"let v{i} = {i}" 
                                               for i in range(random.randint(100, 500))]) + " }",
        
        # Deep call chains
        lambda: generate_deep_call_chain(random.randint(50, 200)),
        
        # Large switch/match
        lambda: "func main() { match x { " + " ".join([f"{i} => {i}," 
                                                        for i in range(random.randint(100, 500))]) + " } }",
    ]
    
    return random.choice(strategies)()

def generate_deep_call_chain(depth: int) -> str:
    """Generate deeply nested function calls."""
    funcs = [f"func f{i}(x: Int) -> Int {{ f{i+1}(x + 1) }}" for i in range(depth - 1)]
    funcs.append(f"func f{depth-1}(x: Int) -> Int {{ x }}")
    funcs.append(f"func main() {{ f0(0) }}")
    return "\n".join(funcs)

def fuzz_all() -> str:
    """Pick a random fuzzing strategy."""
    strategies = [fuzz_lexer, fuzz_parser, fuzz_types, fuzz_codegen]
    return random.choice(strategies)()

# ============================================================================
# Test Runner
# ============================================================================

def run_compiler(input_text: str) -> FuzzResult:
    """Run the Aether compiler on the input and capture results."""
    # Write input to temp file
    with tempfile.NamedTemporaryFile(mode='w', suffix='.aether', delete=False, encoding='utf-8', errors='replace') as f:
        f.write(input_text)
        temp_path = f.name
    
    try:
        result = subprocess.run(
            [AETHERC_PATH, temp_path, "-o", "/tmp/fuzz_out"],
            capture_output=True,
            text=True,
            timeout=TIMEOUT_SECONDS
        )
        
        # Check for crash indicators
        crashed = result.returncode < 0 or "SIGSEGV" in result.stderr or "SIGABRT" in result.stderr
        
        return FuzzResult(
            input_text=input_text,
            exit_code=result.returncode,
            stdout=result.stdout,
            stderr=result.stderr,
            timed_out=False,
            crashed=crashed
        )
    except subprocess.TimeoutExpired:
        return FuzzResult(
            input_text=input_text,
            exit_code=-1,
            stdout="",
            stderr="TIMEOUT",
            timed_out=True,
            crashed=False
        )
    finally:
        os.unlink(temp_path)

def save_crash(result: FuzzResult, crash_type: str) -> str:
    """Save a crash-inducing input for later analysis."""
    os.makedirs(CRASH_DIR, exist_ok=True)
    
    timestamp = int(time.time())
    filename = f"{CRASH_DIR}/{crash_type}_{timestamp}.aether"
    
    with open(filename, 'w') as f:
        f.write(f"// Crash type: {crash_type}\n")
        f.write(f"// Exit code: {result.exit_code}\n")
        f.write(f"// Stderr: {result.stderr[:500]}\n")
        f.write("// ---\n")
        f.write(result.input_text)
    
    return filename

# ============================================================================
# Main Fuzzer
# ============================================================================

def main():
    parser = argparse.ArgumentParser(description="Aether Compiler Fuzzer")
    parser.add_argument("--target", choices=["lexer", "parser", "types", "codegen", "all"],
                        default="all", help="Component to fuzz")
    parser.add_argument("--iterations", type=int, default=1000,
                        help="Number of test cases to generate")
    parser.add_argument("--seed", type=int, default=None,
                        help="Random seed for reproducibility")
    parser.add_argument("--verbose", action="store_true",
                        help="Print each test case")
    
    args = parser.parse_args()
    
    if args.seed is not None:
        random.seed(args.seed)
    
    # Select fuzzer
    fuzzers = {
        "lexer": fuzz_lexer,
        "parser": fuzz_parser,
        "types": fuzz_types,
        "codegen": fuzz_codegen,
        "all": fuzz_all,
    }
    fuzzer = fuzzers[args.target]
    
    # Check compiler exists
    if not os.path.exists(AETHERC_PATH):
        print(f"Error: Compiler not found at {AETHERC_PATH}")
        sys.exit(1)
    
    print(f"🔥 Aether Fuzzer")
    print(f"   Target: {args.target}")
    print(f"   Iterations: {args.iterations}")
    print(f"   Crash dir: {CRASH_DIR}")
    print()
    
    stats = {
        "total": 0,
        "crashes": 0,
        "timeouts": 0,
        "errors": 0,
        "success": 0,
    }
    
    start_time = time.time()
    
    for i in range(args.iterations):
        input_text = fuzzer()
        
        if args.verbose:
            print(f"[{i+1}/{args.iterations}] Testing {len(input_text)} bytes...")
        
        result = run_compiler(input_text)
        stats["total"] += 1
        
        if result.crashed:
            stats["crashes"] += 1
            filename = save_crash(result, "crash")
            print(f"💥 CRASH! Saved to {filename}")
        elif result.timed_out:
            stats["timeouts"] += 1
            filename = save_crash(result, "timeout")
            print(f"⏱️  TIMEOUT! Saved to {filename}")
        elif result.exit_code != 0:
            stats["errors"] += 1
        else:
            stats["success"] += 1
        
        # Progress update
        if (i + 1) % 100 == 0:
            elapsed = time.time() - start_time
            rate = (i + 1) / elapsed
            print(f"   Progress: {i+1}/{args.iterations} ({rate:.1f} tests/sec)")
    
    # Summary
    elapsed = time.time() - start_time
    print()
    print("=" * 50)
    print("📊 FUZZING COMPLETE")
    print(f"   Total: {stats['total']}")
    print(f"   Crashes: {stats['crashes']}")
    print(f"   Timeouts: {stats['timeouts']}")
    print(f"   Errors: {stats['errors']}")
    print(f"   Success: {stats['success']}")
    print(f"   Time: {elapsed:.1f}s ({stats['total']/elapsed:.1f} tests/sec)")
    print()
    
    if stats["crashes"] > 0:
        print(f"⚠️  Found {stats['crashes']} crashes! Check {CRASH_DIR}/")
        sys.exit(1)
    
    print("✅ No crashes found!")
    sys.exit(0)

if __name__ == "__main__":
    main()
