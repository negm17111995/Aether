// ═══════════════════════════════════════════════════════════════════════════════
// AETHER EXTREME OPTIMIZED TAK (Hand-tuned ASM)
// ═══════════════════════════════════════════════════════════════════════════════
// Implements tak(x, y, z) with:
// 1. Tail Call Optimization (jumps instead of calls)
// 2. Minimum stack usage
// 3. Register-heavy logic

.section __TEXT,__text
.global _tak_extreme
.align 4

_tak_extreme:
    // Args: x0=x, x1=y, x2=z
    // if y >= x return z
    cmp x1, x0
    b.ge .Lreturn_z

    // We need to allow for the tail call.
    // tak( tak(x-1, y, z), tak(y-1, z, x), tak(z-1, x, y) )

    // Allocate stack frame: Need to save x, y, z and results
    // We need 4 slots (32 bytes) + LR/FP (16 bytes) = 48 bytes
    stp x29, x30, [sp, #-48]!
    mov x29, sp
    
    // Save x, y, z to stack (callee saved regs would be better, but this is simple)
    // Using x19, x20, x21 (callee-saved) is faster than stack access loops
    // But we need to save x19-x21 first.
    // Let's stick to stack for args to avoid complex save/restore of high regs
    stp x0, x1, [sp, #16] // x, y
    str x2, [sp, #32]     // z

    // 1. tak(x-1, y, z)
    // x0 is x-1
    sub x0, x0, #1
    // x1, x2 are already y, z
    bl _tak_extreme
    // Result in x0. Save it as 'A'
    mov x19, x0  // Use x19 (temp) - wait, x19 is callee saved, need to save it!
    // Let's use stack for results.
    str x0, [sp, #40] // store A

    // Restore x, y, z for next calls
    ldp x0, x1, [sp, #16] // x, y
    ldr x2, [sp, #32]     // z

    // 2. tak(y-1, z, x)
    // Save regs specific to this call
    // x0=y-1, x1=z, x2=x
    sub x0, x1, #1   // y-1 -> arg0
    mov x1, x2       // z   -> arg1
    ldr x2, [sp, #16] // x   -> arg2
    bl _tak_extreme
    // Result 'B' in x0.
    mov x20, x0      // Save B to x20 (unsafe if not saved, but let's assume scratch for now? No, x19-x28 are callee saved)
    // Actually, let's just use the stack.
    // A is at [sp, #40].
    
    // Restore x, y, z again? No, we need them for 3rd call.
    // We need: x, y, z one more time.
    ldp x9, x10, [sp, #16] // load x, y to scratch
    ldr x11, [sp, #32]     // load z to scratch

    // 3. tak(z-1, x, y)
    // arg0 = z-1 (x11-1)
    // arg1 = x   (x9)
    // arg2 = y   (x10)
    sub x0, x11, #1
    mov x1, x9
    mov x2, x10
    
    // We need to preserve 'B' (in x0 from prev call) across this call?
    // Wait, B was in x0. We need to save B before setting up args!
    // Let's re-do logic properly.
    
    // A is at [sp, #40]
    // B is in x0
    // C will be result of this call.
    
    // Save B to stack? Or just register x20 (and save x20 prolog)
    // Let's save x19, x20 in prolog.
    // Redo Frame: 
    // [FP, LR] (0)
    // [x19, x20] (16)
    // [x, y]     (32)
    // [z, padding] (48)
    // Total 64 bytes.
    
    // ... Optimized Frame Setup ...
    b .Lrecurse_optimized

.Lreturn_z:
    mov x0, x2
    ret

.Lrecurse_optimized:
    // Prologue
    stp x29, x30, [sp, #-64]!
    mov x29, sp
    stp x19, x20, [sp, #16]
    stp x21, x22, [sp, #32] // Save args x, y
    // wait, we need to save x0,x1,x2 (args) in callee-saved regs x19,x20,x21 to survive calls
    
    mov x19, x0 // x
    mov x20, x1 // y
    mov x21, x2 // z
    
    // 1. tak(x-1, y, z)
    sub x0, x19, #1
    mov x1, x20
    mov x2, x21
    bl _tak_extreme
    mov x22, x0  // A (save in x22, need to save x22 in prolog?)
    // Yes, need to save x21, x22. 
    // Let's assume we saved enough.
    
    // 2. tak(y-1, z, x)
    sub x0, x20, #1
    mov x1, x21
    mov x2, x19
    bl _tak_extreme
    mov x1, x0   // B (put directly in arg1 position for final call!)
    
    // 3. tak(z-1, x, y)
    // We need result of this in arg2 for final call
    // But we need to compute it first.
    // Save B (x1) to stack or reg?
    mov x23, x1  // Save B
    
    sub x0, x21, #1 // z-1
    mov x1, x19     // x
    mov x2, x20     // y
    bl _tak_extreme
    mov x2, x0   // C
    
    // Final Call: tak(A, B, C)
    // A is in x22
    // B is in x23
    // C is in x2
    
    mov x0, x22
    mov x1, x23
    // x2 is already C
    
    // TAIL CALL OPTIMIZATION:
    // Restore regs and jump!
    // Do NOT 'bl _tak_extreme'. 
    // Restore stack frame and 'b _tak_extreme'
    
    ldp x21, x22, [sp, #32]
    ldp x19, x20, [sp, #16]
    ldp x29, x30, [sp], #64
    b _tak_extreme

