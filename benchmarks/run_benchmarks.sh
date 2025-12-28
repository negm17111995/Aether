#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# AETHER WORLD-CLASS BENCHMARK SUITE
# ═══════════════════════════════════════════════════════════════════════════════
# Compares Aether vs C, C++, C#, Go, Zig, Rust
# Metrics: Time, Memory (RSS), Binary Size, CPU Usage

set -e
cd "$(dirname "$0")"

echo "═══════════════════════════════════════════════════════════════════════════════"
echo "                    AETHER WORLD-CLASS BENCHMARK SUITE"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""

# ═══════════════════════════════════════════════════════════════════════════════
# COMPILE ALL BENCHMARKS
# ═══════════════════════════════════════════════════════════════════════════════

echo "📦 Compiling benchmarks..."
echo ""

# C
echo "  [C]      clang -O3..."
clang -O3 -o tak/tak_c tak/tak.c 2>/dev/null && echo "           ✓ tak_c" || echo "           ✗ FAILED"

# C++
echo "  [C++]    clang++ -O3..."
clang++ -O3 -o tak/tak_cpp tak/tak.cpp 2>/dev/null && echo "           ✓ tak_cpp" || echo "           ✗ FAILED"

# Go
echo "  [Go]     go build..."
(cd tak && go build -o tak_go tak.go 2>/dev/null) && echo "           ✓ tak_go" || echo "           ✗ FAILED"

# Rust
echo "  [Rust]   rustc -O..."
rustc -O -o tak/tak_rust tak/tak.rs 2>/dev/null && echo "           ✓ tak_rust" || echo "           ✗ FAILED"

# Zig
echo "  [Zig]    zig build-exe..."
(cd tak && zig build-exe -O ReleaseFast tak.zig -femit-bin=tak_zig 2>/dev/null) && echo "           ✓ tak_zig" || echo "           ✗ SKIPPED (zig not installed)"

echo ""

# ═══════════════════════════════════════════════════════════════════════════════
# RUN BENCHMARKS
# ═══════════════════════════════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════════════════════════════"
echo "                              TAK BENCHMARK"
echo "                           tak(24, 16, 8)"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""

run_bench() {
    local name=$1
    local cmd=$2
    
    if [ -f "$cmd" ] || command -v "$cmd" &> /dev/null; then
        printf "  %-10s " "[$name]"
        
        # Get timing and memory
        result=$(/usr/bin/time -l "$cmd" 2>&1)
        
        # Extract metrics
        real_time=$(echo "$result" | grep -E "^\s*[0-9]+\.[0-9]+ real" | awk '{print $1}')
        user_time=$(echo "$result" | grep -E "^\s*[0-9]+\.[0-9]+ user" | awk '{print $1}')
        max_rss=$(echo "$result" | grep "maximum resident" | awk '{print $1}')
        
        # Binary size
        if [ -f "$cmd" ]; then
            bin_size=$(ls -l "$cmd" | awk '{print $5}')
            bin_size_kb=$((bin_size / 1024))
        else
            bin_size_kb="N/A"
        fi
        
        printf "Time: %ss  |  RSS: %s KB  |  Binary: %s KB\n" \
               "${real_time:-N/A}" "${max_rss:-N/A}" "$bin_size_kb"
    else
        printf "  %-10s SKIPPED (not compiled)\n" "[$name]"
    fi
}

run_bench "C" "./tak/tak_c"
run_bench "C++" "./tak/tak_cpp"
run_bench "Go" "./tak/tak_go"
run_bench "Rust" "./tak/tak_rust"
run_bench "Zig" "./tak/tak_zig"

echo ""
echo "═══════════════════════════════════════════════════════════════════════════════"
echo "                          BINARY SIZE COMPARISON"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""

ls -lh tak/tak_* 2>/dev/null | awk '{printf "  %-15s %8s\n", $NF, $5}'

echo ""
echo "═══════════════════════════════════════════════════════════════════════════════"
echo "                             FAIL RATE TEST"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""

# TODO: Run fail rate tests (null deref, overflow, OOM)

echo "  [Coming Soon] Fail rate comparison tests"
echo ""
echo "═══════════════════════════════════════════════════════════════════════════════"
