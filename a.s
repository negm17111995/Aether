.section __TEXT,__text,regular,pure_instructions
.global _ast_new
.align 4
_ast_new:
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
    stur x2, [x29, #-24]
    mov x0, #64
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
    mov x21, x0
    mov x0, x21
    add x0, x0, #0
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #8
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #16
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #24
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #32
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #40
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #48
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #56
    mov x20, x0
    mov x19, x0
    mov x0, #0
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_kind
.align 4
_ast_kind:
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
    add x0, x0, #0
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_line
.align 4
_ast_line:
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
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_col
.align 4
_ast_col:
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
    add x0, x0, #16
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_type
.align 4
_ast_type:
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
    add x0, x0, #24
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_data1
.align 4
_ast_data1:
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
    add x0, x0, #32
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_data2
.align 4
_ast_data2:
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
    add x0, x0, #40
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_data3
.align 4
_ast_data3:
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
    add x0, x0, #48
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_data4
.align 4
_ast_data4:
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
    add x0, x0, #56
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_set_type
.align 4
_ast_set_type:
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
    add x0, x0, #24
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_set_data1
.align 4
_ast_set_data1:
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
    add x0, x0, #32
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_set_data2
.align 4
_ast_set_data2:
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
    add x0, x0, #40
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_set_data3
.align 4
_ast_set_data3:
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
    add x0, x0, #48
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_set_data4
.align 4
_ast_set_data4:
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
    add x0, x0, #56
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_int_lit
.align 4
_ast_int_lit:
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
    stur x2, [x29, #-24]
    mov x0, #1
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_float_lit
.align 4
_ast_float_lit:
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
    stur x2, [x29, #-24]
    mov x0, #2
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_str_lit
.align 4
_ast_str_lit:
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
    stur x2, [x29, #-24]
    mov x0, #3
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_bool_lit
.align 4
_ast_bool_lit:
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
    stur x2, [x29, #-24]
    mov x0, #5
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_ident
.align 4
_ast_ident:
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
    stur x2, [x29, #-24]
    mov x0, #10
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_binary
.align 4
_ast_binary:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    stur x4, [x29, #-40]
    mov x0, #11
    mov x19, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    ldur x0, [x29, #-40]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data3
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_unary
.align 4
_ast_unary:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    mov x0, #12
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_call
.align 4
_ast_call:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    mov x0, #13
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_index
.align 4
_ast_index:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    mov x0, #14
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_field
.align 4
_ast_field:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    mov x0, #15
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_let
.align 4
_ast_let:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    stur x4, [x29, #-40]
    stur x5, [x29, #-48]
    mov x0, #30
    mov x19, x0
    ldur x0, [x29, #-40]
    mov x20, x0
    ldur x0, [x29, #-48]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data3
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-32]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data4
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_assign
.align 4
_ast_assign:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    mov x0, #31
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_return
.align 4
_ast_return:
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
    stur x2, [x29, #-24]
    mov x0, #32
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_if
.align 4
_ast_if:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    stur x4, [x29, #-40]
    mov x0, #33
    mov x19, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    ldur x0, [x29, #-40]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data3
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_while
.align 4
_ast_while:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    mov x0, #34
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_for
.align 4
_ast_for:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    stur x4, [x29, #-40]
    mov x0, #35
    mov x19, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    ldur x0, [x29, #-40]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data3
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_block
.align 4
_ast_block:
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
    stur x2, [x29, #-24]
    mov x0, #39
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_expr_stmt
.align 4
_ast_expr_stmt:
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
    stur x2, [x29, #-24]
    mov x0, #38
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_func
.align 4
_ast_func:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    stur x4, [x29, #-40]
    stur x5, [x29, #-48]
    mov x0, #50
    mov x19, x0
    ldur x0, [x29, #-40]
    mov x20, x0
    ldur x0, [x29, #-48]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data3
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-32]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data4
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_struct
.align 4
_ast_struct:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    stur x4, [x29, #-40]
    mov x0, #51
    mov x19, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    ldur x0, [x29, #-40]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data3
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_enum
.align 4
_ast_enum:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    stur x4, [x29, #-40]
    mov x0, #52
    mov x19, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    ldur x0, [x29, #-40]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data3
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_trait
.align 4
_ast_trait:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    stur x4, [x29, #-40]
    mov x0, #53
    mov x19, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    ldur x0, [x29, #-40]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data3
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_import
.align 4
_ast_import:
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
    stur x2, [x29, #-24]
    mov x0, #57
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_const
.align 4
_ast_const:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    stur x4, [x29, #-40]
    mov x0, #56
    mov x19, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    ldur x0, [x29, #-40]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data3
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_module
.align 4
_ast_module:
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
    stur x2, [x29, #-24]
    mov x0, #58
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_type_name
.align 4
_ast_type_name:
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
    stur x2, [x29, #-24]
    mov x0, #70
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_type_ptr
.align 4
_ast_type_ptr:
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
    stur x2, [x29, #-24]
    mov x0, #71
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x20, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_type_array
.align 4
_ast_type_array:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    mov x0, #72
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_type_func
.align 4
_ast_type_func:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    mov x0, #73
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _ast_type_generic
.align 4
_ast_type_generic:
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
    stur x2, [x29, #-24]
    stur x3, [x29, #-32]
    mov x0, #74
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x20, x0
    ldur x0, [x29, #-32]
    mov x20, x0
    mov x0, x19
    mov x1, x20
    mov x2, x20
    bl _ast_new
    mov x21, x0
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data1
    mov x0, x21
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    bl _ast_set_data2
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _param_new
.align 4
_param_new:
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
    stur x2, [x29, #-24]
    mov x0, #24
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
    mov x21, x0
    mov x0, x21
    add x0, x0, #0
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #8
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #16
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _param_name
.align 4
_param_name:
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
    add x0, x0, #0
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _param_type
.align 4
_param_type:
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
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _param_default
.align 4
_param_default:
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
    add x0, x0, #16
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _field_new
.align 4
_field_new:
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
    stur x2, [x29, #-24]
    mov x0, #24
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
    mov x21, x0
    mov x0, x21
    add x0, x0, #0
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-8]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #8
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-16]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add x0, x0, #16
    mov x20, x0
    mov x19, x0
    ldur x0, [x29, #-24]
    mov x1, x0
    mov x0, x20
    str x1, [x0]
    mov x0, x21
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _field_name
.align 4
_field_name:
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
    add x0, x0, #0
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _field_type
.align 4
_field_type:
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
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
.global _field_vis
.align 4
_field_vis:
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
    add x0, x0, #16
    ldr x0, [x0]
    add sp, sp, #256
    ldp x27, x28, [sp, #80]
    ldp x25, x26, [sp, #64]
    ldp x23, x24, [sp, #48]
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #96
    ret
