#!/bin/bash
# Aether Project - Cleanup and Organization Script
# Run this to clean up and organize the project professionally

set -e

echo "=== AETHER PROJECT CLEANUP ==="

# Remove debug/test files
echo "Removing debug files..."
rm -f aetherc_debug aetherc_debug2 aetherc_debug3 aetherc_debug4
rm -f a.out test_open test_open.o test_open.s

# Create professional directories
echo "Creating directories..."
mkdir -p bin docs examples tests

# Copy pure Aether compiler to bin
echo "Setting up bin directory..."
cp aetherc_native bin/aetherc
chmod +x bin/aetherc

# Create .gitignore updates
echo "Updating .gitignore..."
cat >> .gitignore << 'EOF'

# Debug binaries
aetherc_debug*
*.o
*.s
a.out

# Build outputs
bin/
*.out
EOF

echo "=== CLEANUP COMPLETE ==="
echo ""
echo "Project structure:"
ls -la
echo ""
echo "Pure Aether compiler: bin/aetherc"
echo ""
echo "To compile a program:"
echo "  ./bin/aetherc your_program.aether -o output"
