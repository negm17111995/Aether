// C Heavy Benchmark - CPU, Memory, Concurrency
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <pthread.h>
#include <string.h>

int is_prime(int n) {
    if (n < 2) return 0;
    if (n == 2) return 1;
    if (n % 2 == 0) return 0;
    for (int i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return 0;
    }
    return 1;
}

long count_primes(int limit) {
    long count = 0;
    for (int i = 2; i < limit; i++) {
        count += is_prime(i);
    }
    return count;
}

long memory_stress(int allocs) {
    long total = 0;
    for (int i = 0; i < allocs; i++) {
        long *ptr = malloc(1024);
        *ptr = i;
        total += *ptr;
        free(ptr);
    }
    return total;
}

long fib(int n) {
    if (n <= 1) return n;
    long a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        long c = a + b;
        a = b;
        b = c;
    }
    return b;
}

// Concurrent worker
void* worker(void* arg) {
    int id = *(int*)arg;
    long result = count_primes(10000);
    return (void*)result;
}

int main() {
    clock_t start = clock();
    
    // CPU Test
    long primes = count_primes(100000);
    
    // Memory Test
    long mem = memory_stress(10000);
    
    // Compute Test
    long f = fib(40);
    
    // Concurrency Test: 4 threads
    pthread_t threads[4];
    int ids[4] = {1, 2, 3, 4};
    for (int i = 0; i < 4; i++) {
        pthread_create(&threads[i], NULL, worker, &ids[i]);
    }
    for (int i = 0; i < 4; i++) {
        pthread_join(threads[i], NULL);
    }
    
    clock_t end = clock();
    double elapsed = (double)(end - start) / CLOCKS_PER_SEC * 1000;
    
    printf("C Results:\n");
    printf("  Primes: %ld\n", primes);
    printf("  Fib(40): %ld\n", f);
    printf("  Time: %.2f ms\n", elapsed);
    
    return 0;
}
