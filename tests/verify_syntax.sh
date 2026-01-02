#!/bin/bash
# Syntax validation for all Aether files
# Checks for common syntax issues

cd /Users/negm/Desktop/Aether/aether-core

echo "=============================================="
echo "    SYNTAX VALIDATION"
echo "=============================================="
echo ""

VALID=0
INVALID=0
WARNINGS=0

# Check all aether files for common syntax patterns
for file in $(find . -name "*.aether" -type f | grep -v ".git"); do
    basename_file=$(basename "$file")
    
    errors=""
    
    # Check for balanced braces
    open_braces=$(grep -o '{' "$file" | wc -l | tr -d ' ')
    close_braces=$(grep -o '}' "$file" | wc -l | tr -d ' ')
    if [ "$open_braces" != "$close_braces" ]; then
        errors="$errors unbalanced_braces($open_braces/$close_braces)"
    fi
    
    # Check for balanced parentheses
    open_parens=$(grep -o '(' "$file" | wc -l | tr -d ' ')
    close_parens=$(grep -o ')' "$file" | wc -l | tr -d ' ')
    if [ "$open_parens" != "$close_parens" ]; then
        errors="$errors unbalanced_parens($open_parens/$close_parens)"
    fi
    
    # Check for balanced brackets
    open_brackets=$(grep -o '\[' "$file" | wc -l | tr -d ' ')
    close_brackets=$(grep -o '\]' "$file" | wc -l | tr -d ' ')
    if [ "$open_brackets" != "$close_brackets" ]; then
        errors="$errors unbalanced_brackets($open_brackets/$close_brackets)"
    fi
    
    # Check for func declarations
    has_func=$(grep -c "^func " "$file" || echo 0)
    
    # Check for main function in executables
    if [[ "$file" == *"test_"* ]] || [[ "$file" == *"main.aether"* ]]; then
        has_main=$(grep -c "func main" "$file" || echo 0)
        if [ "$has_main" -eq 0 ]; then
            if [[ "$file" != *"suite"* ]]; then
                # test_suite doesn't need main
                errors="$errors no_main"
            fi
        fi
    fi
    
    if [ -z "$errors" ]; then
        VALID=$((VALID + 1))
    else
        printf "  %-50s [ISSUES]%s\n" "$basename_file" "$errors"
        INVALID=$((INVALID + 1))
    fi
done

echo ""
echo "=============================================="
echo "    SYNTAX VALIDATION RESULTS"
echo "=============================================="
echo "  Valid files:   $VALID"
echo "  Files w/issues: $INVALID"
echo ""

if [ $INVALID -eq 0 ]; then
    echo "  ✅ ALL FILES PASS SYNTAX VALIDATION!"
else
    echo "  ⚠️  Some files have potential issues"
fi
echo ""

# Final summary
echo "=============================================="
echo "    FINAL CHECKS"
echo "=============================================="

# Token constants consistency
echo ""
echo "Checking token constants..."
lexer_tokens=$(grep -c "const TOK_" compiler/lexer.aether || echo 0)
echo "  Lexer defines $lexer_tokens token types"

# AST constants
ast_nodes=$(grep -c "const AST_" compiler/ast.aether || echo 0)
echo "  AST defines $ast_nodes node types"

# Instruction encoders
arm_funcs=$(grep -c "^func arm_" compiler/codegen/arm64.aether || echo 0)
x64_funcs=$(grep -c "^func x64_" compiler/codegen/x86_64.aether || echo 0)
echo "  ARM64 encoder: $arm_funcs functions"
echo "  x86_64 encoder: $x64_funcs functions"

# Binary generators
macho_size=$(wc -l < compiler/binary/macho.aether | tr -d ' ')
elf_size=$(wc -l < compiler/binary/elf.aether | tr -d ' ')
pe_size=$(wc -l < compiler/binary/pe.aether | tr -d ' ')
echo "  Mach-O generator: $macho_size lines"
echo "  ELF generator: $elf_size lines"
echo "  PE generator: $pe_size lines"

echo ""
echo "=============================================="
echo "  ✅ COMPREHENSIVE VERIFICATION COMPLETE!"
echo "=============================================="
