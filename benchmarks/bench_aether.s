.section __TEXT,__text,regular,pure_instructions
.global _main
.align 4
_main:
    stp x29, x30, [sp, #-96]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    stp x25, x26, [sp, #64]
    stp x27, x28, [sp, #80]
    mov x29, sp
    sub sp, sp, #256
    mov x0, #40
    bl _fib
    mov x21, x0
    bl _bench_alloc
    mov x22, x0
    bl _bench_concurrent
    mov x23, x0
    bl _sieve
    mov x24, x0
    bl _bench_vector
    mov x25, x0
    mov x0, x21
    mov x19, x0
    mov x0, x24
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _fib
.align 4
_fib:
    stp x29, x30, [sp, #-96]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    stp x25, x26, [sp, #64]
    stp x27, x28, [sp, #80]
    mov x29, sp
    sub sp, sp, #256
    mov x20, x0
    stur x0, [x29, #-8]
    ldur x0, [x29, #-8]
    cmp x0, #1
    cset x0, le
    cbz x0, .L0
    ldur x0, [x29, #-8]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
    b .L1
.L0:
.L1:
    ldur x0, [x29, #-8]
    sub x0, x0, #1
    bl _fib
    mov x19, x0
    ldur x0, [x29, #-8]
    sub x0, x0, #2
    bl _fib
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _bench_alloc
.align 4
_bench_alloc:
    stp x29, x30, [sp, #-96]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    stp x25, x26, [sp, #64]
    stp x27, x28, [sp, #80]
    mov x29, sp
    sub sp, sp, #256
    mov x0, #0
    mov x21, x0
.L2:
    mov x0, x21
    mov x19, x0
    movz x0, #16960
    movk x0, #15, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L3
.L4:
    mov x0, #1024
    add x0, x0, #4096
    and x0, x0, #-4096
    mov x1, x0
    mov x0, #0
    mov x2, #3
    mov x3, #0x1002
    mov x4, #-1
    mov x5, #0
    mov x16, #197
    svc #0x80
    mov x22, x0
    mov x0, x22
    mov x20, x0
    mov x19, x0
    mov x0, x21
    and x0, x0, #255
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x21
    add x0, x0, #1
    mov x21, x0
    mov x0, x21
    mov x19, x0
    movz x0, #16960
    movk x0, #15, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L3
    mov x0, #1024
    add x0, x0, #4096
    and x0, x0, #-4096
    mov x1, x0
    mov x0, #0
    mov x2, #3
    mov x3, #0x1002
    mov x4, #-1
    mov x5, #0
    mov x16, #197
    svc #0x80
    mov x23, x0
    mov x0, x23
    mov x20, x0
    mov x19, x0
    mov x0, x21
    and x0, x0, #255
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x21
    add x0, x0, #1
    mov x21, x0
    mov x0, x21
    mov x19, x0
    movz x0, #16960
    movk x0, #15, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L3
    mov x0, #1024
    add x0, x0, #4096
    and x0, x0, #-4096
    mov x1, x0
    mov x0, #0
    mov x2, #3
    mov x3, #0x1002
    mov x4, #-1
    mov x5, #0
    mov x16, #197
    svc #0x80
    mov x24, x0
    mov x0, x24
    mov x20, x0
    mov x19, x0
    mov x0, x21
    and x0, x0, #255
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x21
    add x0, x0, #1
    mov x21, x0
    mov x0, x21
    mov x19, x0
    movz x0, #16960
    movk x0, #15, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L3
    mov x0, #1024
    add x0, x0, #4096
    and x0, x0, #-4096
    mov x1, x0
    mov x0, #0
    mov x2, #3
    mov x3, #0x1002
    mov x4, #-1
    mov x5, #0
    mov x16, #197
    svc #0x80
    mov x25, x0
    mov x0, x25
    mov x20, x0
    mov x19, x0
    mov x0, x21
    and x0, x0, #255
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x21
    add x0, x0, #1
    mov x21, x0
    mov x0, x21
    mov x19, x0
    movz x0, #16960
    movk x0, #15, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L3
    b .L4
.L3:
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _process_request
.align 4
_process_request:
    stp x29, x30, [sp, #-96]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    stp x25, x26, [sp, #64]
    stp x27, x28, [sp, #80]
    mov x29, sp
    sub sp, sp, #256
    mov x20, x0
    stur x0, [x29, #-8]
    mov x0, #0
    mov x21, x0
    mov x0, #0
    mov x22, x0
.L6:
    mov x0, x22
    mov x19, x0
    mov x0, #10000
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L7
.L8:
    mov x0, x21
    mov x19, x0
    mov x0, x22
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x21, x0
    mov x0, x22
    add x0, x0, #1
    mov x22, x0
    mov x0, x22
    mov x19, x0
    mov x0, #10000
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L7
    mov x0, x21
    mov x19, x0
    mov x0, x22
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x21, x0
    mov x0, x22
    add x0, x0, #1
    mov x22, x0
    mov x0, x22
    mov x19, x0
    mov x0, #10000
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L7
    mov x0, x21
    mov x19, x0
    mov x0, x22
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x21, x0
    mov x0, x22
    add x0, x0, #1
    mov x22, x0
    mov x0, x22
    mov x19, x0
    mov x0, #10000
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L7
    mov x0, x21
    mov x19, x0
    mov x0, x22
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x21, x0
    mov x0, x22
    add x0, x0, #1
    mov x22, x0
    mov x0, x22
    mov x19, x0
    mov x0, #10000
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L7
    b .L8
.L7:
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _bench_concurrent
.align 4
_bench_concurrent:
    stp x29, x30, [sp, #-96]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    stp x25, x26, [sp, #64]
    stp x27, x28, [sp, #80]
    mov x29, sp
    sub sp, sp, #256
    mov x0, #0
    mov x21, x0
    mov x0, #0
    mov x22, x0
.L10:
    mov x0, x22
    mov x19, x0
    movz x0, #34464
    movk x0, #1, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L11
.L12:
    mov x0, x22
    bl _process_request
    mov x23, x0
    mov x0, x23
    cmp x0, #0
    cset x0, gt
    cbz x0, .L14
    mov x0, x21
    add x0, x0, #1
    mov x21, x0
    b .L15
.L14:
.L15:
    mov x0, x22
    add x0, x0, #1
    mov x22, x0
    mov x0, x22
    mov x19, x0
    movz x0, #34464
    movk x0, #1, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L11
    mov x0, x22
    bl _process_request
    mov x24, x0
    mov x0, x24
    cmp x0, #0
    cset x0, gt
    cbz x0, .L16
    mov x0, x21
    add x0, x0, #1
    mov x21, x0
    b .L17
.L16:
.L17:
    mov x0, x22
    add x0, x0, #1
    mov x22, x0
    mov x0, x22
    mov x19, x0
    movz x0, #34464
    movk x0, #1, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L11
    mov x0, x22
    bl _process_request
    mov x25, x0
    mov x0, x25
    cmp x0, #0
    cset x0, gt
    cbz x0, .L18
    mov x0, x21
    add x0, x0, #1
    mov x21, x0
    b .L19
.L18:
.L19:
    mov x0, x22
    add x0, x0, #1
    mov x22, x0
    mov x0, x22
    mov x19, x0
    movz x0, #34464
    movk x0, #1, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L11
    mov x0, x22
    bl _process_request
    mov x26, x0
    mov x0, x26
    cmp x0, #0
    cset x0, gt
    cbz x0, .L20
    mov x0, x21
    add x0, x0, #1
    mov x21, x0
    b .L21
.L20:
.L21:
    mov x0, x22
    add x0, x0, #1
    mov x22, x0
    mov x0, x22
    mov x19, x0
    movz x0, #34464
    movk x0, #1, lsl #16
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L11
    b .L12
.L11:
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _sieve
.align 4
_sieve:
    stp x29, x30, [sp, #-96]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    stp x25, x26, [sp, #64]
    stp x27, x28, [sp, #80]
    mov x29, sp
    sub sp, sp, #256
    movz x0, #38528
    movk x0, #152, lsl #16
    mov x21, x0
    mov x0, x21
    add x0, x0, #4096
    and x0, x0, #-4096
    mov x1, x0
    mov x0, #0
    mov x2, #3
    mov x3, #0x1002
    mov x4, #-1
    mov x5, #0
    mov x16, #197
    svc #0x80
    mov x22, x0
    mov x0, #0
    mov x23, x0
.L22:
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L23
.L24:
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #1
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L23
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #1
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L23
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #1
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L23
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #1
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L23
    b .L24
.L23:
    mov x0, x22
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x22
    add x0, x0, #1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, #2
    mov x23, x0
.L26:
    mov x0, x23
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    mul x0, x0, x1
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L27
.L28:
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldrb w0, [x0]
    cmp x0, #1
    cset x0, eq
    cbz x0, .L30
    mov x0, x23
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    mul x0, x0, x1
    mov x24, x0
.L32:
    mov x0, x24
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L33
.L34:
    mov x0, x22
    mov x19, x0
    mov x0, x24
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x24
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x24, x0
    mov x0, x24
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L33
    mov x0, x22
    mov x19, x0
    mov x0, x24
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x24
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x24, x0
    mov x0, x24
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L33
    mov x0, x22
    mov x19, x0
    mov x0, x24
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x24
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x24, x0
    mov x0, x24
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L33
    mov x0, x22
    mov x19, x0
    mov x0, x24
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x24
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x24, x0
    mov x0, x24
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L33
    b .L34
.L33:
    b .L31
.L30:
.L31:
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    mul x0, x0, x1
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L27
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldrb w0, [x0]
    cmp x0, #1
    cset x0, eq
    cbz x0, .L36
    mov x0, x23
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    mul x0, x0, x1
    mov x25, x0
.L38:
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L39
.L40:
    mov x0, x22
    mov x19, x0
    mov x0, x25
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x25
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L39
    mov x0, x22
    mov x19, x0
    mov x0, x25
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x25
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L39
    mov x0, x22
    mov x19, x0
    mov x0, x25
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x25
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L39
    mov x0, x22
    mov x19, x0
    mov x0, x25
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x25
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L39
    b .L40
.L39:
    b .L37
.L36:
.L37:
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    mul x0, x0, x1
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L27
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldrb w0, [x0]
    cmp x0, #1
    cset x0, eq
    cbz x0, .L42
    mov x0, x23
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    mul x0, x0, x1
    mov x26, x0
.L44:
    mov x0, x26
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L45
.L46:
    mov x0, x22
    mov x19, x0
    mov x0, x26
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x26
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x26, x0
    mov x0, x26
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L45
    mov x0, x22
    mov x19, x0
    mov x0, x26
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x26
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x26, x0
    mov x0, x26
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L45
    mov x0, x22
    mov x19, x0
    mov x0, x26
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x26
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x26, x0
    mov x0, x26
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L45
    mov x0, x22
    mov x19, x0
    mov x0, x26
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x26
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x26, x0
    mov x0, x26
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L45
    b .L46
.L45:
    b .L43
.L42:
.L43:
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    mul x0, x0, x1
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L27
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldrb w0, [x0]
    cmp x0, #1
    cset x0, eq
    cbz x0, .L48
    mov x0, x23
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    mul x0, x0, x1
    mov x27, x0
.L50:
    mov x0, x27
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L51
.L52:
    mov x0, x22
    mov x19, x0
    mov x0, x27
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x27
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x27, x0
    mov x0, x27
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L51
    mov x0, x22
    mov x19, x0
    mov x0, x27
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x27
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x27, x0
    mov x0, x27
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L51
    mov x0, x22
    mov x19, x0
    mov x0, x27
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x27
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x27, x0
    mov x0, x27
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L51
    mov x0, x22
    mov x19, x0
    mov x0, x27
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    strb w1, [x0]
    mov x0, x27
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x27, x0
    mov x0, x27
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L51
    b .L52
.L51:
    b .L49
.L48:
.L49:
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    mul x0, x0, x1
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L27
    b .L28
.L27:
    mov x0, #0
    mov x28, x0
    mov x0, #0
    mov x23, x0
.L54:
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L55
.L56:
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldrb w0, [x0]
    cmp x0, #1
    cset x0, eq
    cbz x0, .L58
    mov x0, x28
    add x0, x0, #1
    mov x28, x0
    b .L59
.L58:
.L59:
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L55
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldrb w0, [x0]
    cmp x0, #1
    cset x0, eq
    cbz x0, .L60
    mov x0, x28
    add x0, x0, #1
    mov x28, x0
    b .L61
.L60:
.L61:
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L55
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldrb w0, [x0]
    cmp x0, #1
    cset x0, eq
    cbz x0, .L62
    mov x0, x28
    add x0, x0, #1
    mov x28, x0
    b .L63
.L62:
.L63:
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L55
    mov x0, x22
    mov x19, x0
    mov x0, x23
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldrb w0, [x0]
    cmp x0, #1
    cset x0, eq
    cbz x0, .L64
    mov x0, x28
    add x0, x0, #1
    mov x28, x0
    b .L65
.L64:
.L65:
    mov x0, x23
    add x0, x0, #1
    mov x23, x0
    mov x0, x23
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L55
    b .L56
.L55:
    mov x0, x28
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _bench_vector
.align 4
_bench_vector:
    stp x29, x30, [sp, #-96]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    stp x25, x26, [sp, #64]
    stp x27, x28, [sp, #80]
    mov x29, sp
    sub sp, sp, #256
    movz x0, #38528
    movk x0, #152, lsl #16
    mov x21, x0
    mov x0, x21
    lsl x0, x0, #3
    add x0, x0, #4096
    and x0, x0, #-4096
    mov x1, x0
    mov x0, #0
    mov x2, #3
    mov x3, #0x1002
    mov x4, #-1
    mov x5, #0
    mov x16, #197
    svc #0x80
    mov x22, x0
    mov x0, x21
    lsl x0, x0, #3
    add x0, x0, #4096
    and x0, x0, #-4096
    mov x1, x0
    mov x0, #0
    mov x2, #3
    mov x3, #0x1002
    mov x4, #-1
    mov x5, #0
    mov x16, #197
    svc #0x80
    mov x23, x0
    mov x0, x21
    lsl x0, x0, #3
    add x0, x0, #4096
    and x0, x0, #-4096
    mov x1, x0
    mov x0, #0
    mov x2, #3
    mov x3, #0x1002
    mov x4, #-1
    mov x5, #0
    mov x16, #197
    svc #0x80
    mov x24, x0
    mov x0, #0
    mov x25, x0
.L66:
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L67
.L68:
    mov x0, x22
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x25
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x23
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #1
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x25
    add x0, x0, #1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L67
    mov x0, x22
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x25
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x23
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #1
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x25
    add x0, x0, #1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L67
    mov x0, x22
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x25
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x23
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #1
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x25
    add x0, x0, #1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L67
    mov x0, x22
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x25
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x23
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #1
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x25
    add x0, x0, #1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L67
    b .L68
.L67:
    mov x0, #0
    mov x25, x0
.L70:
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L71
.L72:
    mov x0, x22
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldr x0, [x0]
    mov x26, x0
    mov x0, x23
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldr x0, [x0]
    mov x27, x0
    mov x0, x24
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x26
    mov x19, x0
    mov x0, x27
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x25
    add x0, x0, #1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L71
    mov x0, x22
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldr x0, [x0]
    mov x28, x0
    mov x0, x23
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldr x0, [x0]
    stur x0, [x29, #-8]
    mov x0, x24
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x28
    mov x19, x0
    mov x0, x27
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x25
    add x0, x0, #1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L71
    mov x0, x22
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldr x0, [x0]
    stur x0, [x29, #-16]
    mov x0, x23
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldr x0, [x0]
    stur x0, [x29, #-24]
    mov x0, x24
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x28
    mov x19, x0
    mov x0, x27
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x25
    add x0, x0, #1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L71
    mov x0, x22
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldr x0, [x0]
    stur x0, [x29, #-32]
    mov x0, x23
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    ldr x0, [x0]
    stur x0, [x29, #-40]
    mov x0, x24
    mov x19, x0
    mov x0, x25
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    mov x0, x28
    mov x19, x0
    mov x0, x27
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x25
    add x0, x0, #1
    mov x25, x0
    mov x0, x25
    mov x19, x0
    mov x0, x21
    mov x1, x0
    mov x0, x19
    cmp x0, x1
    cset x0, lt
    cbz x0, .L71
    b .L72
.L71:
    mov x0, x24
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
