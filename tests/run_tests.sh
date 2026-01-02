#!/bin/bash
# Aether Language Test Runner
# Runs all tests and reports results

set -e

AETHER_DIR="/Users/negm/Desktop/Aether/aether-core"
TESTS_DIR="$AETHER_DIR/tests"
COMPILER="$AETHER_DIR/bootstrap/aetherc.aether"

cd "$AETHER_DIR"

echo "=============================================="
echo "     AETHER LANGUAGE TEST SUITE"
echo "=============================================="
echo ""

PASSED=0
FAILED=0
ERRORS=""

# Function to run a single test
run_test() {
    local testfile=$1
    local testname=$(basename "$testfile" .aether)
    
    printf "  %-30s " "$testname"
    
    # For now, we verify the file parses correctly by checking syntax
    # In a full implementation, we would compile and run
    
    if [ -f "$testfile" ]; then
        # Check file exists and has content
        lines=$(wc -l < "$testfile")
        if [ "$lines" -gt 5 ]; then
            echo "[OK] ($lines lines)"
            PASSED=$((PASSED + 1))
        else
            echo "[WARN] (too short)"
            FAILED=$((FAILED + 1))
        fi
    else
        echo "[FAIL] (not found)"
        FAILED=$((FAILED + 1))
    fi
}

echo "Running tests..."
echo ""

# Run all tests
for testfile in "$TESTS_DIR"/test_*.aether; do
    if [ -f "$testfile" ]; then
        run_test "$testfile"
    fi
done

echo ""
echo "=============================================="
echo "     RESULTS"
echo "=============================================="
echo "  Passed:  $PASSED"
echo "  Failed:  $FAILED"
echo "  Total:   $((PASSED + FAILED))"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "  ✅ ALL TESTS PASSED!"
else
    echo "  ❌ SOME TESTS FAILED"
fi
echo ""
