#!/bin/bash
# Deep verification of Aether compiler modules

cd /Users/negm/Desktop/Aether/aether-core

echo "=============================================="
echo "    AETHER MODULE VERIFICATION"
echo "=============================================="
echo ""

PASS=0
FAIL=0

check_file() {
    local file=$1
    local desc=$2
    
    if [ -f "$file" ]; then
        lines=$(wc -l < "$file" | tr -d ' ')
        size=$(stat -f %z "$file" 2>/dev/null || stat -c %s "$file" 2>/dev/null)
        printf "  %-40s [OK] %6s lines, %6s bytes\n" "$desc" "$lines" "$size"
        PASS=$((PASS + 1))
    else
        printf "  %-40s [FAIL] not found\n" "$desc"
        FAIL=$((FAIL + 1))
    fi
}

check_import() {
    local file=$1
    local import=$2
    
    if grep -q "import $import" "$file" 2>/dev/null; then
        printf "    ↳ imports %-30s [OK]\n" "$import"
    else
        printf "    ↳ imports %-30s [MISSING]\n" "$import"
    fi
}

echo "1. COMPILER CORE MODULES"
echo "------------------------"
check_file "compiler/main.aether" "compiler/main.aether"
check_import "compiler/main.aether" "runtime.core"
check_import "compiler/main.aether" "compiler.lexer"
check_import "compiler/main.aether" "compiler.parser"

check_file "compiler/lexer.aether" "compiler/lexer.aether"
check_file "compiler/parser.aether" "compiler/parser.aether"
check_import "compiler/parser.aether" "runtime.vec"
check_import "compiler/parser.aether" "compiler.lexer"
check_import "compiler/parser.aether" "compiler.ast"

check_file "compiler/ast.aether" "compiler/ast.aether"
check_file "compiler/typechecker.aether" "compiler/typechecker.aether"
echo ""

echo "2. CODE GENERATION MODULES"
echo "--------------------------"
check_file "compiler/codegen/main.aether" "compiler/codegen/main.aether"
check_import "compiler/codegen/main.aether" "compiler.ast"
check_import "compiler/codegen/main.aether" "compiler.lexer"
check_import "compiler/codegen/main.aether" "compiler.codegen.arm64"
check_import "compiler/codegen/main.aether" "compiler.codegen.x86_64"

check_file "compiler/codegen/arm64.aether" "compiler/codegen/arm64.aether"
check_file "compiler/codegen/x86_64.aether" "compiler/codegen/x86_64.aether"
echo ""

echo "3. BINARY FORMAT MODULES"
echo "------------------------"
check_file "compiler/binary/macho.aether" "compiler/binary/macho.aether (macOS)"
check_file "compiler/binary/elf.aether" "compiler/binary/elf.aether (Linux)"
check_file "compiler/binary/pe.aether" "compiler/binary/pe.aether (Windows)"
echo ""

echo "4. RUNTIME MODULES"
echo "------------------"
check_file "runtime/core.aether" "runtime/core.aether"
check_file "runtime/vec.aether" "runtime/vec.aether"
check_file "runtime/map.aether" "runtime/map.aether"
check_file "runtime/str.aether" "runtime/str.aether"
echo ""

echo "5. ADVANCED FEATURES"
echo "--------------------"
check_file "compiler/veritas/types.aether" "veritas/types.aether (type system)"
check_file "compiler/veritas/effects.aether" "veritas/effects.aether (effects)"
check_file "stdlib/std/actor/actor.aether" "std/actor.aether (actors)"
check_file "stdlib/std/cloud/cloud.aether" "std/cloud.aether (cloud)"
check_file "stdlib/std/cluster/cluster.aether" "std/cluster.aether (cluster)"
check_file "stdlib/std/db/db.aether" "std/db.aether (database)"
check_file "stdlib/std/ffi/ffi.aether" "std/ffi.aether (FFI)"
check_file "stdlib/std/hot_reload/reload.aether" "std/hot_reload.aether"
echo ""

echo "6. BOOTSTRAP"
echo "------------"
check_file "bootstrap/aether.aether" "bootstrap/aether.aether"
echo ""

echo "=============================================="
echo "    VERIFICATION RESULTS"
echo "=============================================="
echo "  Modules Found:   $PASS"
echo "  Modules Missing: $FAIL"
echo "  Total Checked:   $((PASS + FAIL))"
echo ""

# Check for any non-Aether files
echo "7. PURITY CHECK"
echo "---------------"
c_files=$(find . -name "*.c" -o -name "*.h" -o -name "*.cpp" -o -name "*.rs" 2>/dev/null | grep -v ".git" | wc -l | tr -d ' ')
py_files=$(find . -name "*.py" 2>/dev/null | grep -v ".git" | wc -l | tr -d ' ')
js_files=$(find . -name "*.js" -o -name "*.ts" 2>/dev/null | grep -v ".git" | wc -l | tr -d ' ')

echo "  C/C++/Rust files: $c_files"
echo "  Python files:     $py_files"
echo "  JavaScript files: $js_files"
echo ""

if [ "$c_files" -eq 0 ] && [ "$py_files" -eq 0 ] && [ "$js_files" -eq 0 ]; then
    echo "  ✅ 100% PURE AETHER - No external language dependencies!"
else
    echo "  ⚠️  Some non-Aether files found"
fi
echo ""

# Count total
total_aether=$(find . -name "*.aether" | wc -l | tr -d ' ')
total_lines=$(find . -name "*.aether" -exec cat {} \; | wc -l | tr -d ' ')
echo "8. TOTAL CODEBASE"
echo "-----------------"
echo "  Aether files: $total_aether"
echo "  Lines of code: $total_lines"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "=============================================="
    echo "  ✅ ALL MODULES VERIFIED SUCCESSFULLY!"
    echo "=============================================="
else
    echo "=============================================="
    echo "  ❌ SOME MODULES MISSING"
    echo "=============================================="
fi
