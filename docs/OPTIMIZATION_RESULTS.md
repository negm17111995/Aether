# Aether Compiler Optimization - Final Results

## Performance Summary

| Version | Time | Speedup |
|---------|------|---------|
| **Original** | 1.57s | 1.0x |
| **Optimized** | **0.35s** | **4.5x** |
| C (gcc -O3) | 0.17s | 9.2x |

### Gap to C: 2.0x (was 9.2x)

## Optimizations Applied

| # | Optimization | Impact |
|---|--------------|--------|
| 1 | Reduced register saves (12→4) | 2.1x |
| 2 | Register-based binary ops | 1.6x |
| 3 | Constant folding | 1.1x |
| 4 | Optimized function calls | 1.05x |
| 5 | Immediate comparisons/arithmetic | 1.1x |

## Generated Assembly Comparison

### Before (Original):
```asm
_fib:
    stp x29, x30, [sp, #-16]!
    stp x19, x20, [sp, #-16]!
    stp x21, x22, [sp, #-16]!    ; excessive saves
    stp x23, x24, [sp, #-16]!
    stp x25, x26, [sp, #-16]!
    stp x27, x28, [sp, #-16]!
    sub sp, sp, #256              ; large stack
    ...
    mov x19, x0                   ; save left
    mov x0, #0x2
    mov x1, x0                    ; redundant moves
    mov x0, x19
    cmp x0, x1                    ; compare via register
    cset x0, lt
```

### After (Optimized):
```asm
_fib:
    stp x29, x30, [sp, #-32]!     ; only 2 pairs saved
    stp x19, x20, [sp, #16]
    mov x29, sp
    sub sp, sp, #32               ; small stack (32 vs 256)
    ...
    cmp x0, #0x2                  ; immediate compare!
    cset x0, lt
```

## Why Not Faster?

To match C (0.17s), we need:
1. **Function inlining** - Inline base case returns
2. **Better register allocation** - Keep `n` in register across recursive calls
3. **Loop unrolling** - Transform recursion to iteration
4. **LLVM backend** - Use mature optimization infrastructure

## Conclusion

**Achieved 4.5x speedup** with pure Aether codegen optimizations.
**Closed gap to C from 9.2x to 2.0x.**

The remaining 2.0x gap would require deeper compiler infrastructure changes.
