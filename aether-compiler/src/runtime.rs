//! Aether Runtime - Built-in functions and memory management

use std::collections::HashMap;

/// Runtime built-in functions
pub struct Runtime {
    /// Memory heap
    heap: Vec<u8>,
    /// Heap pointer
    heap_ptr: usize,
}

impl Runtime {
    pub fn new(heap_size: usize) -> Self {
        Runtime {
            heap: vec![0; heap_size],
            heap_ptr: 0,
        }
    }
    
    /// Allocate memory
    pub fn malloc(&mut self, size: usize) -> usize {
        let ptr = self.heap_ptr;
        self.heap_ptr += size;
        // Align to 8 bytes
        self.heap_ptr = (self.heap_ptr + 7) & !7;
        ptr
    }
    
    /// Free memory (no-op for bump allocator)
    pub fn free(&mut self, _ptr: usize) {
        // Bump allocator doesn't free
    }
}

/// List of built-in functions
pub fn builtins() -> HashMap<&'static str, (&'static str, Vec<&'static str>)> {
    let mut m = HashMap::new();
    
    // Memory
    m.insert("__builtin_malloc", ("Int", vec!["Int"]));
    m.insert("__builtin_free", ("()", vec!["Int"]));
    m.insert("__builtin_store8", ("()", vec!["Int", "Int"]));
    m.insert("__builtin_store64", ("()", vec!["Int", "Int"]));
    m.insert("__builtin_load8", ("Int", vec!["Int"]));
    m.insert("__builtin_load64", ("Int", vec!["Int"]));
    
    // I/O
    m.insert("__builtin_print", ("()", vec!["Int"]));
    m.insert("__builtin_open", ("Int", vec!["Int", "Int", "Int"]));
    m.insert("__builtin_read", ("Int", vec!["Int", "Int", "Int"]));
    m.insert("__builtin_write", ("Int", vec!["Int", "Int", "Int"]));
    m.insert("__builtin_close", ("Int", vec!["Int"]));
    
    // Process
    m.insert("__builtin_exit", ("!", vec!["Int"]));
    
    m
}
