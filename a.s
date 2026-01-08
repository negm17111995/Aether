.section __TEXT,__text,regular,pure_instructions
.global _repl_new
.align 4
_repl_new:
    stp x29, x30, [sp, #-96]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    stp x25, x26, [sp, #64]
    stp x27, x28, [sp, #80]
    mov x29, sp
    sub sp, sp, #256
    mov x0, #16
    bl _ae_malloc
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    bl _vec_new
    mov x1, x0
    mov x0, x20
    bl _ae_store64
    mov x0, x21
    add x0, x0, #8
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    bl _ae_store64
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _repl_add_history
.align 4
_repl_add_history:
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
    stur x1, [x29, #-16]
    ldur x0, [x29, #-8]
    bl _ae_load64
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _vec_push
    ldur x0, [x29, #-8]
    add x0, x0, #8
    bl _ae_load64
    mov x22, x0
    ldur x0, [x29, #-8]
    add x0, x0, #8
    mov x20, x0
    mov x19, x0
    mov x0, x22
    add x0, x0, #1
    mov x1, x0
    mov x0, x20
    bl _ae_store64
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _repl_get_history
.align 4
_repl_get_history:
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
    stur x1, [x29, #-16]
    ldur x0, [x29, #-8]
    bl _ae_load64
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _vec_get
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _repl_history_count
.align 4
_repl_history_count:
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
    add x0, x0, #8
    bl _ae_load64
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _repl_eval
.align 4
_repl_eval:
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
    stur x1, [x29, #-16]
    ldur x0, [x29, #-8]
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _repl_add_history
    mov x0, #0
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _rvec_new
.align 4
_rvec_new:
    stp x29, x30, [sp, #-96]!
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32]
    stp x23, x24, [sp, #48]
    stp x25, x26, [sp, #64]
    stp x27, x28, [sp, #80]
    mov x29, sp
    sub sp, sp, #256
    mov x0, #24
    bl _ae_malloc
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    mov x0, #512
    bl _ae_malloc
    mov x1, x0
    mov x0, x20
    bl _ae_store64
    mov x0, x21
    add x0, x0, #8
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    bl _ae_store64
    mov x0, x21
    add x0, x0, #16
    mov x20, x0
    mov x19, x0
    mov x0, #64
    mov x1, x0
    mov x0, x20
    bl _ae_store64
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _rvec_push
.align 4
_rvec_push:
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
    stur x1, [x29, #-16]
    ldur x0, [x29, #-8]
    bl _ae_load64
    mov x21, x0
    ldur x0, [x29, #-8]
    add x0, x0, #8
    bl _ae_load64
    mov x22, x0
    mov x0, x21
    mov x19, x0
    mov x0, x22
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ae_store64
    ldur x0, [x29, #-8]
    add x0, x0, #8
    mov x20, x0
    mov x19, x0
    mov x0, x22
    add x0, x0, #1
    mov x1, x0
    mov x0, x20
    bl _ae_store64
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _rvec_len
.align 4
_rvec_len:
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
    add x0, x0, #8
    bl _ae_load64
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _rvec_get
.align 4
_rvec_get:
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
    stur x1, [x29, #-16]
    ldur x0, [x29, #-8]
    bl _ae_load64
    mov x21, x0
    mov x0, x21
    mov x19, x0
    ldur x0, [x29, #-16]
    lsl x0, x0, #3
    mov x1, x0
    mov x0, x19
    add x0, x0, x1
    bl _ae_load64
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
