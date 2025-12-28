// STRESS TEST - Stack Depth
// Pushes recursion until stack overflow

#include <stdio.h>

volatile long depth = 0;

void recurse() {
    depth++;
    if (depth % 100000 == 0) {
        printf("Depth: %ld\n", depth);
    }
    recurse();
}

int main() {
    printf("C Stack Depth Stress Test\n");
    recurse();
    return 0;
}
