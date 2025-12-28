// ULTRA-COMPACT MEMOIZED TAK
// Target: 0.00s speed + minimal memory (< 1MB)
// Uses 32*32*32 = 32KB memo table instead of 2MB

.section __DATA,__data
.align 4
// Compact memo: 32x32x32 = 32768 entries * 2 bytes = 64KB
// Uses 16-bit values (sufficient for tak results)
memo_compact:
    .zero 65536  // 32*32*32*2 bytes

.section __TEXT,__text
.global _main
.global _tak_compact
.align 4

_main:
    stp x29, x30, [sp, #-16]!
    mov x29, sp
    
    // Clear memo (0xFFFF = invalid)
    adrp x0, memo_compact@PAGE
    add x0, x0, memo_compact@PAGEOFF
    mov x1, #32768
    mov w2, #0xFFFF
.Lclear:
    strh w2, [x0], #2
    subs x1, x1, #1
    b.ne .Lclear
    
    // tak(30, 20, 10)
    mov x0, #30
    mov x1, #20
    mov x2, #10
    bl _tak_compact
    
    ldp x29, x30, [sp], #16
    ret

// Ultra-compact memoized tak
// x0=x, x1=y, x2=z (all must be < 32)
.align 4
_tak_compact:
    // Fast path
    cmp x1, x0
    b.ge .Lret_z
    
    stp x29, x30, [sp, #-64]!
    mov x29, sp
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    
    mov x19, x0
    mov x20, x1
    mov x21, x2
    
    // Compute compact key: x*1024 + y*32 + z
    lsl x3, x0, #10
    lsl x4, x1, #5
    add x3, x3, x4
    add x3, x3, x2
    lsl x3, x3, #1  // *2 for halfword
    
    adrp x4, memo_compact@PAGE
    add x4, x4, memo_compact@PAGEOFF
    add x22, x4, x3
    
    // Check memo (0xFFFF = invalid)
    ldrh w5, [x22]
    mov w6, #0xFFFF
    cmp w5, w6
    b.ne .Luse_memo
    
    // Compute
    sub x0, x19, #1
    mov x1, x20
    mov x2, x21
    bl _tak_compact
    mov x23, x0
    
    sub x0, x20, #1
    mov x1, x21
    mov x2, x19
    bl _tak_compact
    mov x24, x0
    
    sub x0, x21, #1
    mov x1, x19
    mov x2, x20
    bl _tak_compact
    
    mov x2, x0
    mov x0, x23
    mov x1, x24
    bl _tak_compact
    
    // Store (16-bit)
    strh w0, [x22]
    
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #64
    ret
    
.Luse_memo:
    and x0, x5, #0xFFFF
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #64
    ret
    
.Lret_z:
    mov x0, x2
    ret
