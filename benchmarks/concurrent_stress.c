// C Concurrent Stress Test - 100 threads each allocating heavily
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#define NUM_THREADS 100
#define ALLOCS_PER_THREAD 1000
#define ALLOC_SIZE 65536  // 64KB per alloc

typedef struct {
    int id;
    long success;
    int failed;
} ThreadResult;

void* worker(void* arg) {
    ThreadResult* result = (ThreadResult*)arg;
    result->success = 0;
    result->failed = 0;
    
    for (int i = 0; i < ALLOCS_PER_THREAD; i++) {
        void* ptr = malloc(ALLOC_SIZE);
        if (ptr) {
            memset(ptr, result->id & 0xFF, ALLOC_SIZE);
            result->success++;
            free(ptr);
        } else {
            result->failed = 1;
            break;
        }
    }
    
    return NULL;
}

int main() {
    printf("C Concurrent Stress: %d threads x %d allocations x %d KB\n", 
           NUM_THREADS, ALLOCS_PER_THREAD, ALLOC_SIZE/1024);
    
    pthread_t threads[NUM_THREADS];
    ThreadResult results[NUM_THREADS];
    
    // Launch all threads
    for (int i = 0; i < NUM_THREADS; i++) {
        results[i].id = i;
        if (pthread_create(&threads[i], NULL, worker, &results[i]) != 0) {
            printf("FAILED to create thread %d\n", i);
            return 1;
        }
    }
    
    // Wait for all
    long total_success = 0;
    int failed_threads = 0;
    for (int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
        total_success += results[i].success;
        if (results[i].failed) failed_threads++;
    }
    
    printf("Total allocations: %ld / %d\n", total_success, NUM_THREADS * ALLOCS_PER_THREAD);
    printf("Failed threads: %d / %d\n", failed_threads, NUM_THREADS);
    
    if (failed_threads > 0) {
        printf("STATUS: FAILED (%d threads hit OOM)\n", failed_threads);
        return 1;
    }
    printf("STATUS: PASSED\n");
    return 0;
}
