//! Aether Runtime - Optimized Memory Management
//! Arena allocator with free list for memory reuse

use std::collections::HashMap;

/// Runtime with optimized memory allocator
pub struct Runtime {
    /// Memory heap - pre-allocated
    heap: Vec<u8>,
    /// Current heap pointer
    heap_ptr: usize,
    /// Free list: size -> list of free block pointers
    free_list: HashMap<usize, Vec<usize>>,
    /// Block sizes for tracking
    block_sizes: HashMap<usize, usize>,
}

impl Runtime {
    pub fn new(heap_size: usize) -> Self {
        Runtime {
            heap: vec![0; heap_size],
            heap_ptr: 0,
            free_list: HashMap::new(),
            block_sizes: HashMap::new(),
        }
    }
    
    /// Allocate memory - reuses freed blocks when possible
    pub fn malloc(&mut self, size: usize) -> usize {
        // Align to 8 bytes
        let aligned_size = (size + 7) & !7;
        
        // Check free list for reusable block
        if let Some(blocks) = self.free_list.get_mut(&aligned_size) {
            if let Some(ptr) = blocks.pop() {
                return ptr;
            }
        }
        
        // Check for larger free blocks we can split
        for (&block_size, blocks) in self.free_list.iter_mut() {
            if block_size >= aligned_size && !blocks.is_empty() {
                if let Some(ptr) = blocks.pop() {
                    self.block_sizes.insert(ptr, aligned_size);
                    return ptr;
                }
            }
        }
        
        // Allocate new from heap
        let ptr = self.heap_ptr;
        self.heap_ptr += aligned_size;
        self.block_sizes.insert(ptr, aligned_size);
        ptr
    }
    
    /// Free memory - adds to free list for reuse
    pub fn free(&mut self, ptr: usize) {
        if let Some(&size) = self.block_sizes.get(&ptr) {
            self.free_list
                .entry(size)
                .or_insert_with(Vec::new)
                .push(ptr);
        }
    }
    
    /// Reset allocator - instant cleanup
    pub fn reset(&mut self) {
        self.heap_ptr = 0;
        self.free_list.clear();
        self.block_sizes.clear();
    }
}

/// List of built-in functions with types
pub fn builtins() -> HashMap<&'static str, (&'static str, Vec<&'static str>)> {
    let mut m = HashMap::new();
    
    // Memory - optimized
    m.insert("__builtin_malloc", ("Int", vec!["Int"]));
    m.insert("__builtin_free", ("()", vec!["Int"]));
    m.insert("__builtin_store8", ("()", vec!["Int", "Int"]));
    m.insert("__builtin_store16", ("()", vec!["Int", "Int"]));
    m.insert("__builtin_store32", ("()", vec!["Int", "Int"]));
    m.insert("__builtin_store64", ("()", vec!["Int", "Int"]));
    m.insert("__builtin_load8", ("Int", vec!["Int"]));
    m.insert("__builtin_load16", ("Int", vec!["Int"]));
    m.insert("__builtin_load32", ("Int", vec!["Int"]));
    m.insert("__builtin_load64", ("Int", vec!["Int"]));
    m.insert("__builtin_memcpy", ("()", vec!["Int", "Int", "Int"]));
    m.insert("__builtin_memset", ("()", vec!["Int", "Int", "Int"]));
    
    // I/O
    m.insert("__builtin_print", ("()", vec!["Int"]));
    m.insert("__builtin_open", ("Int", vec!["Int", "Int", "Int"]));
    m.insert("__builtin_read", ("Int", vec!["Int", "Int", "Int"]));
    m.insert("__builtin_write", ("Int", vec!["Int", "Int", "Int"]));
    m.insert("__builtin_close", ("Int", vec!["Int"]));
    m.insert("__builtin_lseek", ("Int", vec!["Int", "Int", "Int"]));
    
    // Network
    m.insert("__builtin_socket", ("Int", vec!["Int", "Int", "Int"]));
    m.insert("__builtin_connect", ("Int", vec!["Int", "Int", "Int"]));
    m.insert("__builtin_send", ("Int", vec!["Int", "Int", "Int", "Int"]));
    m.insert("__builtin_recv", ("Int", vec!["Int", "Int", "Int", "Int"]));
    
    // Process
    m.insert("__builtin_exit", ("!", vec!["Int"]));
    m.insert("__builtin_fork", ("Int", vec![]));
    m.insert("__builtin_exec", ("Int", vec!["Int"]));
    
    // Filesystem
    m.insert("__builtin_stat", ("Int", vec!["Int", "Int"]));
    m.insert("__builtin_mkdir", ("Int", vec!["Int", "Int"]));
    m.insert("__builtin_rmdir", ("Int", vec!["Int"]));
    m.insert("__builtin_unlink", ("Int", vec!["Int"]));
    m.insert("__builtin_rename", ("Int", vec!["Int", "Int"]));
    m.insert("__builtin_opendir", ("Int", vec!["Int"]));
    m.insert("__builtin_readdir", ("Int", vec!["Int"]));
    m.insert("__builtin_closedir", ("Int", vec!["Int"]));
    
    m
}
