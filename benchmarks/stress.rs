// Rust Extreme Stress Test
use std::env;

const ALLOC_SIZE: usize = 1048576; // 1MB

fn test_allocations(count: usize) -> usize {
    let mut vecs: Vec<Vec<u8>> = Vec::with_capacity(count);
    let mut success = 0;
    
    for i in 0..count {
        match std::panic::catch_unwind(|| {
            vec![0u8; ALLOC_SIZE]
        }) {
            Ok(v) => {
                vecs.push(v);
                success += 1;
            }
            Err(_) => {
                println!("  FAILED at allocation #{}", i);
                break;
            }
        }
    }
    
    success
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(1000)
    } else {
        1000
    };
    
    println!("Rust Stress Test: {} x 1MB allocations", target);
    
    let success = test_allocations(target);
    
    println!("Result: {}/{} allocations succeeded", success, target);
    println!("Memory used: {} MB", success);
    
    if success < target {
        println!("STATUS: FAILED at {}", success);
        std::process::exit(1);
    }
    println!("STATUS: PASSED");
}
