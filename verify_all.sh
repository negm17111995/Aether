#!/bin/bash
# Verify all Aether files
echo "═══════════════════════════════════════════════════════════════"
echo "VERIFYING ALL 158 AETHER FILES"
echo "═══════════════════════════════════════════════════════════════"

total=0
passed=0
failed=0

for file in $(find . -name "*.aether" -not -path "./stdlib_full/*" | sort); do
    echo -n "Checking $file ... "
    # Run compiler only for syntax/parsing check (no output)
    ./aetherc "$file" > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        echo "✓ OK"
        passed=$((passed+1))
    else
        echo "❌ FAIL"
        # Run again to show error
        ./aetherc "$file"
        failed=$((failed+1))
    fi
    total=$((total+1))
done

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "SUMMARY: Passed: $passed / $total"
echo "═══════════════════════════════════════════════════════════════"
if [ $failed -eq 0 ]; then
    exit 0
else
    exit 1
fi
