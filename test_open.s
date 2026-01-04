// Test file reading - debug version
.section __TEXT,__text,regular,pure_instructions

.global _main

.align 4

// print(char)
_print:
    stp x29, x30, [sp, #-16]!
    sub sp, sp, #16
    strb w0, [sp]
    mov x0, #1
    mov x1, sp
    mov x2, #1
    mov x16, #4
    svc #0x80
    add sp, sp, #16
    ldp x29, x30, [sp], #16
    ret

_main:
    stp x29, x30, [sp, #-16]!
    
    // Get argv[1]
    ldr x0, [x1, #8]       // x0 = argv[1] (file path)
    cbz x0, .Lno_arg
    
    // Try to open file
    mov x1, #0             // O_RDONLY
    mov x16, #5            // SYS_open
    svc #0x80
    
    // Check if open succeeded
    cmp x0, #0
    blt .Lopen_failed
    
    // Print 'O' for open success
    mov x0, #79
    bl _print
    
    mov x0, #0
    ldp x29, x30, [sp], #16
    ret
    
.Lno_arg:
    mov x0, #78  // 'N'
    bl _print
    mov x0, #1
    ldp x29, x30, [sp], #16
    ret
    
.Lopen_failed:
    mov x0, #70  // 'F'
    bl _print
    mov x0, #1
    ldp x29, x30, [sp], #16
    ret
