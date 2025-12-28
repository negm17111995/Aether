// Hand-optimized Tak for ARM64
// Passes args directly in registers, no stack spilling

.section __TEXT,__text
.global _main
.align 4

_main:
    stp x29, x30, [sp, #-16]!
    mov x29, sp
    
    mov x0, #30
    mov x1, #20
    mov x2, #10
    bl _tak
    
    ldp x29, x30, [sp], #16
    ret

.global _tak
.align 4
_tak:
    // Args: x0=x, x1=y, x2=z
    // if y >= x return z
    cmp x1, x0
    b.ge .Lret_z
    
    // Need to save x, y, z for multiple recursive calls
    stp x29, x30, [sp, #-64]!
    mov x29, sp
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    str x23, [sp, #48]
    
    // Save original args
    mov x19, x0  // x
    mov x20, x1  // y
    mov x21, x2  // z
    
    // Call 1: tak(x-1, y, z)
    sub x0, x19, #1
    mov x1, x20
    mov x2, x21
    bl _tak
    mov x22, x0  // Result A
    
    // Call 2: tak(y-1, z, x)
    sub x0, x20, #1
    mov x1, x21
    mov x2, x19
    bl _tak
    mov x23, x0  // Result B
    
    // Call 3: tak(z-1, x, y)
    sub x0, x21, #1
    mov x1, x19
    mov x2, x20
    bl _tak
    // Result C in x0
    mov x2, x0
    
    // Final call: tak(A, B, C)
    mov x0, x22
    mov x1, x23
    // x2 already has C
    
    // Restore and tail-call
    ldr x23, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #64
    b _tak
    
.Lret_z:
    mov x0, x2
    ret
