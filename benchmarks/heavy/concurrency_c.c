// Concurrency Benchmark - Lock-Free Counter
// Tests: Atomic operations, minimal contention

#include <stdio.h>
#include <pthread.h>
#include <stdatomic.h>
#include <time.h>

#define NUM_THREADS 8
#define ITERS 10000000

atomic_long counter = 0;

void* increment(void* arg) {
    for (int i = 0; i < ITERS; i++) {
        atomic_fetch_add(&counter, 1);
    }
    return NULL;
}

int main() {
    pthread_t threads[NUM_THREADS];
    clock_t start = clock();
    
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_create(&threads[i], NULL, increment, NULL);
    }
    
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }
    
    clock_t end = clock();
    double time = (double)(end - start) / CLOCKS_PER_SEC;
    
    printf("Counter: %ld\n", counter);
    printf("Time: %.3f seconds\n", time);
    printf("Ops/sec: %.0f million\n", (NUM_THREADS * ITERS / time) / 1000000);
    
    return 0;
}
