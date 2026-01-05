#!/bin/bash
# AETHER CLEANUP SCRIPT
# Run this to remove all duplicates and organize the project
# Usage: chmod +x cleanup.sh && ./cleanup.sh

set -e
cd /Users/negm/Desktop/Aether/aether-core

echo "=== AETHER PROJECT CLEANUP ==="
echo ""

# 1. Remove duplicate compiler
echo "1. Removing stdlib/aether_compiler/ (duplicate of compiler/)..."
rm -rf stdlib/aether_compiler

# 2. Remove debug files
echo "2. Removing debug binaries..."
rm -f aetherc_debug aetherc_debug2 aetherc_debug3 aetherc_debug4

# 3. Remove test files
echo "3. Removing test files..."
rm -f a.out test_open test_open.o test_open.s

# 4. Create bin directory
echo "4. Setting up bin directory..."
mkdir -p bin
cp aetherc_native bin/aetherc
chmod +x bin/aetherc

# 5. Count remaining files
echo ""
echo "=== CLEANUP COMPLETE ==="
echo "Aether modules remaining: $(find . -name '*.aether' -type f | wc -l)"
echo ""
echo "Project structure:"
ls -la
echo ""
echo "Use ./bin/aetherc to compile Aether programs"
