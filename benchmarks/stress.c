// C Extreme Stress Test - Push until failure
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#define ALLOC_SIZE 1048576  // 1MB per allocation

long test_allocations(int count) {
    long success = 0;
    void** ptrs = malloc(count * sizeof(void*));
    if (!ptrs) return 0;
    
    for (int i = 0; i < count; i++) {
        ptrs[i] = malloc(ALLOC_SIZE);
        if (ptrs[i]) {
            memset(ptrs[i], i & 0xFF, ALLOC_SIZE);
            success++;
        } else {
            printf("  FAILED at allocation #%d\n", i);
            break;
        }
    }
    
    // Free all
    for (int i = 0; i < success; i++) {
        free(ptrs[i]);
    }
    free(ptrs);
    
    return success;
}

void* thread_worker(void* arg) {
    int id = *(int*)arg;
    long allocs = test_allocations(100);
    return (void*)allocs;
}

int main(int argc, char** argv) {
    int target = 1000;
    if (argc > 1) target = atoi(argv[1]);
    
    printf("C Stress Test: %d x 1MB allocations\n", target);
    
    long success = test_allocations(target);
    
    printf("Result: %ld/%d allocations succeeded\n", success, target);
    printf("Memory used: %ld MB\n", success);
    
    if (success < target) {
        printf("STATUS: FAILED at %ld\n", success);
        return 1;
    }
    printf("STATUS: PASSED\n");
    return 0;
}
