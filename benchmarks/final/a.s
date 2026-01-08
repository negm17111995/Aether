.section __TEXT,__text,regular,pure_instructions
.global _main
.align 4
_main:
    stp x20, x19, [sp, #-32]!
    stp x29, x30, [sp, #16]
    add x29, sp, #16
    sub sp, sp, #128
    stp x20, x19, [sp, #-32]!
    stp x29, x30, [sp, #16]
    add x29, sp, #16
    sub sp, sp, #128
    mov x0, #10
    bl _fib
    mov x0, #0
    add sp, sp, #128
    ldp x29, x30, [sp, #16]
    ldp x20, x19, [sp], #32
    ret
.global _fib
.align 4
_fib:
    stp x20, x19, [sp, #-32]!
    stp x29, x30, [sp, #16]
    add x29, sp, #16
    sub sp, sp, #128
    stp x20, x19, [sp, #-32]!
    stp x29, x30, [sp, #16]
    add x29, sp, #16
    sub sp, sp, #128
    mov x21, x0
    mov x0, x21
    cmp x0, #1
    cset x0, le
    cbz x0, .L0
    mov x0, x21
    add sp, sp, #128
    ldp x29, x30, [sp, #16]
    ldp x20, x19, [sp], #32
    ret
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
    add sp, sp, #128
    ldp x29, x30, [sp, #16]
    ldp x20, x19, [sp], #32
    ret
