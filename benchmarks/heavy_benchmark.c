// C HEAVY BENCHMARK - Comparison with Aether
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

// ============================================================================
// TEST 1: FIBONACCI
// ============================================================================

long long fib(int n) {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

long bench_fib() {
    clock_t start = clock();
    long long result = fib(40);
    clock_t end = clock();
    long elapsed = (end - start) * 1000 / CLOCKS_PER_SEC;
    printf("Fibonacci(40) = %lld in %ldms\n", result, elapsed);
    return elapsed;
}

// ============================================================================
// TEST 2: MEMORY ALLOCATION
// ============================================================================

long bench_alloc(int count) {
    clock_t start = clock();
    for (int i = 0; i < count; i++) {
        void* ptr = malloc(1024);
        *(long*)ptr = i;
    }
    clock_t end = clock();
    long elapsed = (end - start) * 1000 / CLOCKS_PER_SEC;
    printf("Allocated %d x 1KB blocks in %ldms\n", count, elapsed);
    return elapsed;
}

// ============================================================================
// TEST 3: CONCURRENT SIMULATION
// ============================================================================

long process_request(int id) {
    long sum = 0;
    for (int i = 0; i < 10000; i++) {
        sum += i;
    }
    return sum;
}

long bench_concurrent(int requests) {
    clock_t start = clock();
    int completed = 0;
    
    for (int i = 0; i < requests; i++) {
        long result = process_request(i);
        if (result > 0) completed++;
    }
    
    clock_t end = clock();
    long elapsed = (end - start) * 1000 / CLOCKS_PER_SEC;
    printf("Processed %d requests in %ldms\n", completed, elapsed);
    return elapsed;
}

// ============================================================================
// TEST 4: PRIME SIEVE
// ============================================================================

int sieve(int limit) {
    char* is_prime = malloc(limit);
    memset(is_prime, 1, limit);
    is_prime[0] = is_prime[1] = 0;
    
    for (int i = 2; i * i < limit; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j < limit; j += i) {
                is_prime[j] = 0;
            }
        }
    }
    
    int count = 0;
    for (int i = 0; i < limit; i++) {
        if (is_prime[i]) count++;
    }
    free(is_prime);
    return count;
}

long bench_sieve() {
    clock_t start = clock();
    int primes = sieve(10000000);
    clock_t end = clock();
    long elapsed = (end - start) * 1000 / CLOCKS_PER_SEC;
    printf("Found %d primes under 10M in %ldms\n", primes, elapsed);
    return elapsed;
}

// ============================================================================
// TEST 5: VECTOR OPERATIONS
// ============================================================================

long bench_vector(int size) {
    clock_t start = clock();
    
    long* a = malloc(size * sizeof(long));
    long* b = malloc(size * sizeof(long));
    long* c = malloc(size * sizeof(long));
    
    for (int i = 0; i < size; i++) {
        a[i] = i;
        b[i] = i * 2;
    }
    
    for (int i = 0; i < size; i++) {
        c[i] = a[i] + b[i];
    }
    
    clock_t end = clock();
    long elapsed = (end - start) * 1000 / CLOCKS_PER_SEC;
    printf("Vector add of %d elements in %ldms\n", size, elapsed);
    
    free(a); free(b); free(c);
    return elapsed;
}

// ============================================================================
// MAIN
// ============================================================================

int main() {
    printf("=== C HEAVY BENCHMARK ===\n\n");
    
    printf("TEST 1: Fibonacci (Recursive)\n");
    bench_fib();
    
    printf("\nTEST 2: Memory Allocation (1M blocks)\n");
    bench_alloc(1000000);
    
    printf("\nTEST 3: Concurrent Requests (100K)\n");
    bench_concurrent(100000);
    
    printf("\nTEST 4: Prime Sieve (10M)\n");
    bench_sieve();
    
    printf("\nTEST 5: Vector Operations (10M)\n");
    bench_vector(10000000);
    
    printf("\n=== BENCHMARK COMPLETE ===\n");
    return 0;
}
