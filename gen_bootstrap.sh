#!/bin/bash
# PURE AETHER BOOTSTRAP GENERATOR
# Uses ONLY shell built-ins and printf to create ARM64 Mach-O
# NO C, NO PYTHON, NO OTHER LANGUAGES

set -e
echo "=== PURE AETHER BOOTSTRAP ==="
echo "Generating ARM64 Mach-O binary using pure shell..."

OUT="bootstrap/aetherc"

# Function to write bytes
w() {
    printf '%b' "$1" >> "$OUT"
}

# Start fresh
rm -f "$OUT"

# --- Mach-O Header (32 bytes) ---
# Magic: 0xFEEDFACF (little endian)
w '\xCF\xFA\xED\xFE'
# CPU Type: ARM64 (0x0100000C)
w '\x0C\x00\x00\x01'
# CPU Subtype: 0
w '\x00\x00\x00\x00'
# Filetype: MH_EXECUTE (2)
w '\x02\x00\x00\x00'
# Ncmds: 3
w '\x03\x00\x00\x00'
# Sizeofcmds: 72 + 152 + 280 = 504
w '\xF8\x01\x00\x00'
# Flags: MH_NOUNDEFS (1)
w '\x01\x00\x00\x00'
# Reserved
w '\x00\x00\x00\x00'

# --- LC_SEGMENT_64: __PAGEZERO (72 bytes) ---
# LC_SEGMENT_64 = 0x19
w '\x19\x00\x00\x00'
# Cmdsize: 72
w '\x48\x00\x00\x00'
# Segname: __PAGEZERO (16 bytes)
w '__PAGEZERO\x00\x00\x00\x00\x00\x00'
# vmaddr: 0
w '\x00\x00\x00\x00\x00\x00\x00\x00'
# vmsize: 0x100000000 (4GB)
w '\x00\x00\x00\x00\x01\x00\x00\x00'
# fileoff: 0
w '\x00\x00\x00\x00\x00\x00\x00\x00'
# filesize: 0
w '\x00\x00\x00\x00\x00\x00\x00\x00'
# maxprot: 0
w '\x00\x00\x00\x00'
# initprot: 0
w '\x00\x00\x00\x00'
# nsects: 0
w '\x00\x00\x00\x00'
# flags: 0
w '\x00\x00\x00\x00'

# --- LC_SEGMENT_64: __TEXT (72 + 80 = 152 bytes) ---
w '\x19\x00\x00\x00'
# Cmdsize: 152
w '\x98\x00\x00\x00'
# Segname: __TEXT (16 bytes)
w '__TEXT\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
# vmaddr: 0x100000000
w '\x00\x00\x00\x00\x01\x00\x00\x00'
# vmsize: 0x2000 (8KB)
w '\x00\x20\x00\x00\x00\x00\x00\x00'
# fileoff: 0
w '\x00\x00\x00\x00\x00\x00\x00\x00'
# filesize: 0x1030 (code offset + code size)
w '\x30\x10\x00\x00\x00\x00\x00\x00'
# maxprot: r-x (5)
w '\x05\x00\x00\x00'
# initprot: r-x (5)
w '\x05\x00\x00\x00'
# nsects: 1
w '\x01\x00\x00\x00'
# flags: 0
w '\x00\x00\x00\x00'

# --- Section: __text (80 bytes) ---
# sectname: __text
w '__text\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
# segname: __TEXT
w '__TEXT\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
# addr: 0x100001000
w '\x00\x10\x00\x00\x01\x00\x00\x00'
# size: 48 bytes (code + string)
w '\x30\x00\x00\x00\x00\x00\x00\x00'
# offset: 0x1000 (4096)
w '\x00\x10\x00\x00'
# align: 2 (4 bytes)
w '\x02\x00\x00\x00'
# reloff: 0
w '\x00\x00\x00\x00'
# nreloc: 0
w '\x00\x00\x00\x00'
# flags: S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS
w '\x00\x04\x00\x80'
# reserved1-3
w '\x00\x00\x00\x00'
w '\x00\x00\x00\x00'
w '\x00\x00\x00\x00'

# --- LC_UNIXTHREAD (280 bytes) ---
w '\x05\x00\x00\x00'
# Cmdsize: 280
w '\x18\x01\x00\x00'
# Flavor: ARM_THREAD_STATE64 (6)
w '\x06\x00\x00\x00'
# Count: 68
w '\x44\x00\x00\x00'
# x0-x28 (29 * 8 = 232 bytes) - all zeros
for i in {1..29}; do w '\x00\x00\x00\x00\x00\x00\x00\x00'; done
# x29 (fp)
w '\x00\x00\x00\x00\x00\x00\x00\x00'
# x30 (lr)
w '\x00\x00\x00\x00\x00\x00\x00\x00'
# sp
w '\x00\x00\x00\x00\x00\x00\x00\x00'
# pc = entry point = 0x100001000
w '\x00\x10\x00\x00\x01\x00\x00\x00'
# cpsr + pad
w '\x00\x00\x00\x00\x00\x00\x00\x00'

# --- Pad to 0x1000 (4096) ---
CURRENT=536
while [ $CURRENT -lt 4096 ]; do
    w '\x00'
    CURRENT=$((CURRENT + 1))
done

# --- ARM64 Code ---
# mov x0, 1 (stdout)
w '\x20\x00\x80\xD2'
# adr x1, #44 (address of string, 11 instructions ahead)
w '\x81\x05\x00\x10'
# mov x2, 8 (length)
w '\x02\x01\x80\xD2'
# mov x16, 4 (write syscall)
w '\x90\x00\x80\xD2'
# svc #0x80
w '\x01\x10\x00\xD4'
# mov x0, 0 (exit code)
w '\x00\x00\x80\xD2'
# mov x16, 1 (exit syscall)
w '\x30\x00\x80\xD2'
# svc #0x80
w '\x01\x10\x00\xD4'

# String: "AETHERC\n" (8 bytes)
w 'AETHERC\n'

# Pad to align
while [ $(wc -c < "$OUT") -lt 4144 ]; do
    w '\x00'
done

chmod +x "$OUT"

echo "Testing..."
./"$OUT"
RESULT=$?

echo ""
if [ $RESULT -eq 0 ]; then
    echo "✅ SUCCESS!"
    file "$OUT"
    ls -la "$OUT"
else
    echo "Exit code: $RESULT"
fi
