// SIMD Vector Sum Benchmark - Aether vs C
// Tests: NEON vectorization on ARM64

.section __TEXT,__text
.global _main
.global _vec_sum
.align 4

// Sum 10 million integers using SIMD
_main:
    stp x29, x30, [sp, #-16]!
    mov x29, sp
    
    // Allocate 10M * 8 bytes = 80MB
    mov x0, #80000000
    bl _malloc_stub
    mov x19, x0  // Array pointer
    
    // Fill with values 1-10M
    mov x1, #10000000
    mov x2, #0
.Lfill:
    add x2, x2, #1
    str x2, [x19, x2, lsl #3]
    cmp x2, x1
    b.lt .Lfill
    
    // Sum using SIMD
    mov x0, x19
    mov x1, #10000000
    bl _vec_sum
    
    ldp x29, x30, [sp], #16
    ret

// SIMD vector sum - 4 elements at a time
// x0 = array, x1 = count
_vec_sum:
    movi v0.2d, #0  // Accumulator
    mov x2, #0      // Index
    
.Lloop:
    ld1 {v1.2d}, [x0], #16  // Load 2 x 64-bit
    add v0.2d, v0.2d, v1.2d
    add x2, x2, #2
    cmp x2, x1
    b.lt .Lloop
    
    // Horizontal sum
    addp d0, v0.2d
    fmov x0, d0
    ret

_malloc_stub:
    mov x16, #0xC5  // mmap
    mov x1, #3      // PROT_READ | PROT_WRITE
    mov x2, #0x1001 // MAP_ANON | MAP_PRIVATE
    mov x3, #-1
    mov x4, #0
    svc #0x80
    ret
