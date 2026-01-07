#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

long fib(long n) {
    if (n <= 1) return n;
    return fib(n-1) + fib(n-2);
}

long bench_fib() {
    return fib(42);
}

long bench_alloc() {
    long count = 10000000;
    long i;
    long sum = 0;
    for (i = 0; i < count; i++) {
        long* ptr = (long*)malloc(64);
        if (!ptr) {
             printf("Malloc failed at %ld\n", i);
             exit(1);
        }
        *ptr = i;
        sum += *ptr;
        // No free - match Aether behavior
    }
    return sum;
}

long process_request(long id) {
    long acc = 0;
    long i;
    for (i = 0; i < 1000; i++) {
        acc += i * id;
    }
    return acc;
}

long bench_requests() {
    long total = 1000000;
    long i;
    long completed = 0;
    for (i = 0; i < total; i++) {
        if (process_request(i) > 0) {
            completed++;
        }
    }
    return completed;
}

long bench_sieve() {
    long limit = 50000000;
    char* is_prime = (char*)malloc(limit);
    long i, j;
    
    for (i = 0; i < limit; i++) is_prime[i] = 1;
    
    for (i = 2; i * i < limit; i++) {
        if (is_prime[i]) {
            for (j = i * i; j < limit; j += i) {
                is_prime[j] = 0;
            }
        }
    }
    
    long count = 0;
    for (i = 2; i < limit; i++) {
        if (is_prime[i]) count++;
    }
    return count;
}

int main() {
    long f = bench_fib();
    long a = bench_alloc();
    long r = bench_requests();
    long s = bench_sieve();
    return f + a + r + s; // inhibit optimization
}
