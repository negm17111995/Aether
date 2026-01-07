fn fib(n: i64) -> i64 {
    if n <= 1 { return n; }
    fib(n - 1) + fib(n - 2)
}

fn bench_fib() -> i64 {
    fib(42)
}

fn bench_alloc() -> i64 {
    let count = 10000000;
    let mut sum = 0;
    for i in 0..count {
        let b = Box::new(i);
        sum += *b;
        std::mem::forget(b); // No free, leak like Aether
    }
    sum
}

fn process_request(id: i64) -> i64 {
    let mut acc = 0;
    for i in 0..1000 {
        acc += i * id;
    }
    acc
}

fn bench_requests() -> i64 {
    let total = 1000000;
    let mut completed = 0;
    for i in 0..total {
        if process_request(i) > 0 {
            completed += 1;
        }
    }
    completed
}

fn bench_sieve() -> i64 {
    let limit = 50000000;
    let mut is_prime = vec![true; limit];
    
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
    
    let mut count = 0;
    for i in 2..limit {
        if is_prime[i] { count += 1; }
    }
    count
}

fn main() {
    let f = bench_fib();
    let a = bench_alloc();
    let r = bench_requests();
    let s = bench_sieve();
    // inhibit optimization
    println!("{}", f + a + r + s); 
}
