#include <iostream>
#include <vector>
#include <numeric>

long fib(long n) {
    if (n <= 1) return n;
    return fib(n-1) + fib(n-2);
}

long bench_fib() {
    return fib(42);
}

long bench_alloc() {
    long count = 10000000;
    long sum = 0;
    for (long i = 0; i < count; i++) {
        long* ptr = new long;
        *ptr = i;
        sum += *ptr;
        // No free
    }
    return sum;
}

long process_request(long id) {
    long acc = 0;
    for (long i = 0; i < 1000; i++) {
        acc += i * id;
    }
    return acc;
}

long bench_requests() {
    long total = 1000000;
    long completed = 0;
    for (long i = 0; i < total; i++) {
        if (process_request(i) > 0) {
            completed++;
        }
    }
    return completed;
}

long bench_sieve() {
    long limit = 50000000;
    std::vector<char> is_prime(limit, 1);
    
    for (long i = 2; i * i < limit; i++) {
        if (is_prime[i]) {
            for (long j = i * i; j < limit; j += i) {
                is_prime[j] = 0;
            }
        }
    }
    
    long count = 0;
    for (long i = 2; i < limit; i++) {
        if (is_prime[i]) count++;
    }
    return count;
}

int main() {
    long f = bench_fib();
    long a = bench_alloc();
    long r = bench_requests();
    long s = bench_sieve();
    return (int)(f + a + r + s);
}
