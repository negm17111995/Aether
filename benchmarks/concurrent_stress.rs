// Rust Concurrent Stress - 100 threads with heavy allocation
use std::thread;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::sync::Arc;

const NUM_THREADS: usize = 100;
const ALLOCS_PER_THREAD: usize = 1000;
const ALLOC_SIZE: usize = 65536;

fn main() {
    println!("Rust Concurrent Stress: {} threads x {} allocations x {} KB",
             NUM_THREADS, ALLOCS_PER_THREAD, ALLOC_SIZE / 1024);
    
    let total_success = Arc::new(AtomicUsize::new(0));
    let failed_threads = Arc::new(AtomicUsize::new(0));
    
    let handles: Vec<_> = (0..NUM_THREADS).map(|id| {
        let success = Arc::clone(&total_success);
        let failed = Arc::clone(&failed_threads);
        
        thread::spawn(move || {
            let result = std::panic::catch_unwind(|| {
                for _ in 0..ALLOCS_PER_THREAD {
                    let mut v = vec![0u8; ALLOC_SIZE];
                    v[0] = id as u8;
                    success.fetch_add(1, Ordering::SeqCst);
                }
            });
            
            if result.is_err() {
                failed.fetch_add(1, Ordering::SeqCst);
            }
        })
    }).collect();
    
    for h in handles {
        let _ = h.join();
    }
    
    let total = total_success.load(Ordering::SeqCst);
    let fails = failed_threads.load(Ordering::SeqCst);
    
    println!("Total allocations: {} / {}", total, NUM_THREADS * ALLOCS_PER_THREAD);
    println!("Failed threads: {} / {}", fails, NUM_THREADS);
    
    if fails > 0 {
        println!("STATUS: FAILED ({} threads panicked)", fails);
        std::process::exit(1);
    }
    println!("STATUS: PASSED");
}
