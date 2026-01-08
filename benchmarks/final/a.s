.section __TEXT,__text,regular,pure_instructions
.global _main
.align 4
_main:
    stp x29, x30, [sp, #-48]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    mov x29, sp
    mov x0, #40
    bl _fib
    mov x21, x0
    mov x0, x21
    sub sp, sp, #16
    strb w0, [sp]
    mov x2, #1
    mov x1, sp
    mov x0, #1
    mov x16, #4
    svc #0x80
    add sp, sp, #16
    mov x0, #0
.Lret_main:
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #48
    ret
.global _fib
.align 4
_fib:
    stp x29, x30, [sp, #-48]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    mov x29, sp
    mov x21, x0
    mov x0, x21
    cmp x0, #1
    cset x0, le
    cbz x0, .L0
    mov x0, x21
    b .Lret_fib
    b .L1
.L0:
.L1:
    mov x0, x21
    sub x0, x0, #1
    bl _fib
    mov x19, x0
    mov x0, x21
    sub x0, x0, #2
    bl _fib
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
.Lret_fib:
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #48
    ret
