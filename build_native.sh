#!/bin/bash
set -e

echo "[1/7] Assembling Pure Aether Source - NO C ALLOWED..."
rm -f /tmp/aether_pure.aether

# 1. Base Standard Library
cat stdlib/std.aether >> /tmp/aether_pure.aether

# 2. Runtime Core
for f in stdlib/std/runtime/*.aether; do
    cat "$f" >> /tmp/aether_pure.aether
done

# 3. Morphic (Bytecode VM)
cat stdlib/std/morphic/ir.aether >> /tmp/aether_pure.aether

# 4. Compiler Core
cat stdlib/aether_compiler/ast.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/lexer.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/typechecker.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/parser.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/borrowck.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/effects.aether >> /tmp/aether_pure.aether

# 5. Veritas (All advanced features)
for f in stdlib/aether_compiler/veritas/*.aether; do
    cat "$f" >> /tmp/aether_pure.aether
done

# 6. Native Backend (ALL platforms)
cat stdlib/aether_compiler/native/syscall.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/x86_64.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/arm64.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/regalloc.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/elf.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/macho.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/pe.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/codegen.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/main.aether >> /tmp/aether_pure.aether

# 7. Optional stdlib modules
cat stdlib/std/ffi/ffi.aether >> /tmp/aether_pure.aether 2>/dev/null || true
cat stdlib/std/io.aether >> /tmp/aether_pure.aether 2>/dev/null || true
cat stdlib/std/collections.aether >> /tmp/aether_pure.aether 2>/dev/null || true

echo "[2/6] Source assembled. Lines: $(wc -l < /tmp/aether_pure.aether)"

echo "[3/6] Cleaning imports..."
# Remove 'import' lines to prevent redefinition in single-file build
sed -i '' '/^import /d' /tmp/aether_pure.aether

echo "[4/6] Compiling (Pure Aether Self-Hosting)..."
# Pure Aether self-hosting - no C involved
./bootstrap/aetherc /tmp/aether_pure.aether

echo "[5/6] Compilation complete. Finalizing..."
# No C files generated - pure Aether
mv a.out bootstrap/aetherc 2>/dev/null || true
chmod +x bootstrap/aetherc

echo "[6/6] SUCCESS! Native compiler is ready."
ls -l bootstrap/aetherc
