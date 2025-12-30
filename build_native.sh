#!/bin/bash
set -e

echo "[1/6] Assembling Source..."
rm -f /tmp/aether_pure.aether

# 1. Base
cat stdlib/std.aether >> /tmp/aether_pure.aether

# 2. Compiler Core
cat stdlib/aether_compiler/ast.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/lexer.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/typechecker.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/parser.aether >> /tmp/aether_pure.aether

# 3. Veritas (All features)
for f in stdlib/aether_compiler/veritas/*.aether; do
    cat "$f" >> /tmp/aether_pure.aether
done

# 4. Native Backend
cat stdlib/aether_compiler/native/syscall.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/x86_64.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/arm64.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/regalloc.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/elf.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/macho.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/pe.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/codegen.aether >> /tmp/aether_pure.aether
cat stdlib/aether_compiler/native/main.aether >> /tmp/aether_pure.aether

echo "[2/6] Source assembled. Lines: $(wc -l < /tmp/aether_pure.aether)"

echo "[3/6] Cleaning imports..."
# Remove 'import' lines to prevent redefinition in single-file build
sed -i '' '/^import /d' /tmp/aether_pure.aether

echo "[4/6] Compiling (Pure Aether Self-Hosting)..."
# Pure Aether self-hosting - no C involved
./bootstrap/aetherc_native /tmp/aether_pure.aether

echo "[5/6] Compilation complete. Finalizing..."
# No C files generated - pure Aether
mv a.out bootstrap/aetherc_native
chmod +x bootstrap/aetherc_native

echo "[6/6] SUCCESS! Native compiler is ready."
ls -l bootstrap/aetherc_native
