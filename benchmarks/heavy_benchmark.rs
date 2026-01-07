// Rust HEAVY BENCHMARK
use std::time::Instant;

// ============================================================================
// TEST 1: FIBONACCI
// ============================================================================

fn fib(n: i64) -> i64 {
    if n <= 1 { return n; }
    fib(n - 1) + fib(n - 2)
}

fn bench_fib() -> u128 {
    let start = Instant::now();
    let result = fib(40);
    let elapsed = start.elapsed().as_millis();
    println!("Fibonacci(40) = {} in {}ms", result, elapsed);
    elapsed
}

// ============================================================================
// TEST 2: MEMORY ALLOCATION
// ============================================================================

fn bench_alloc(count: usize) -> u128 {
    let start = Instant::now();
    let mut vecs: Vec<Vec<u8>> = Vec::with_capacity(count);
    for i in 0..count {
        let mut v = vec![0u8; 1024];
        v[0] = (i % 256) as u8;
        vecs.push(v);
    }
    let elapsed = start.elapsed().as_millis();
    println!("Allocated {} x 1KB blocks in {}ms", count, elapsed);
    elapsed
}

// ============================================================================
// TEST 3: CONCURRENT SIMULATION
// ============================================================================

fn process_request(id: usize) -> i64 {
    let mut sum: i64 = 0;
    for i in 0..10000 {
        sum += i;
    }
    sum
}

fn bench_concurrent(requests: usize) -> u128 {
    let start = Instant::now();
    let mut completed = 0;
    
    for i in 0..requests {
        let result = process_request(i);
        if result > 0 { completed += 1; }
    }
    
    let elapsed = start.elapsed().as_millis();
    println!("Processed {} requests in {}ms", completed, elapsed);
    elapsed
}

// ============================================================================
// TEST 4: PRIME SIEVE
// ============================================================================

fn sieve(limit: usize) -> usize {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;
    
    let mut i = 2;
    while i * i < limit {
        if is_prime[i] {
            let mut j = i * i;
            while j < limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    
    is_prime.iter().filter(|&&x| x).count()
}

fn bench_sieve() -> u128 {
    let start = Instant::now();
    let primes = sieve(10_000_000);
    let elapsed = start.elapsed().as_millis();
    println!("Found {} primes under 10M in {}ms", primes, elapsed);
    elapsed
}

// ============================================================================
// TEST 5: VECTOR OPERATIONS
// ============================================================================

fn bench_vector(size: usize) -> u128 {
    let start = Instant::now();
    
    let a: Vec<i64> = (0..size as i64).collect();
    let b: Vec<i64> = (0..size as i64).map(|x| x * 2).collect();
    let c: Vec<i64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
    
    let elapsed = start.elapsed().as_millis();
    println!("Vector add of {} elements in {}ms (sum check: {})", size, elapsed, c[0]);
    elapsed
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== RUST HEAVY BENCHMARK ===\n");
    
    println!("TEST 1: Fibonacci (Recursive)");
    bench_fib();
    
    println!("\nTEST 2: Memory Allocation (1M blocks)");
    bench_alloc(1_000_000);
    
    println!("\nTEST 3: Concurrent Requests (100K)");
    bench_concurrent(100_000);
    
    println!("\nTEST 4: Prime Sieve (10M)");
    bench_sieve();
    
    println!("\nTEST 5: Vector Operations (10M)");
    bench_vector(10_000_000);
    
    println!("\n=== BENCHMARK COMPLETE ===");
}
