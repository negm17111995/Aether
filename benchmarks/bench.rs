// Rust Heavy Benchmark - CPU, Memory, Concurrency
use std::time::Instant;
use std::thread;

fn is_prime(n: i64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

fn count_primes(limit: i64) -> i64 {
    (2..limit).filter(|&n| is_prime(n)).count() as i64
}

fn memory_stress(allocs: i32) -> i64 {
    let mut total: i64 = 0;
    for i in 0..allocs {
        let v: Vec<u8> = vec![0u8; 1024];
        total += i as i64;
        drop(v);
    }
    total
}

fn fib(n: i64) -> i64 {
    if n <= 1 { return n; }
    let (mut a, mut b) = (0i64, 1i64);
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    let start = Instant::now();
    
    // CPU Test
    let primes = count_primes(100000);
    
    // Memory Test
    let mem = memory_stress(10000);
    
    // Compute Test
    let f = fib(40);
    
    // Concurrency Test: 4 threads
    let handles: Vec<_> = (0..4).map(|_| {
        thread::spawn(|| count_primes(10000))
    }).collect();
    
    for h in handles {
        let _ = h.join();
    }
    
    let elapsed = start.elapsed().as_millis();
    
    println!("Rust Results:");
    println!("  Primes: {}", primes);
    println!("  Fib(40): {}", f);
    println!("  Time: {} ms", elapsed);
}
