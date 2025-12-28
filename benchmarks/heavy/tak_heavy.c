// C Heavy Benchmark - Tak Recursion
#include <stdio.h>

int tak(int x, int y, int z) {
    if (y >= x) return z;
    return tak(
        tak(x - 1, y, z),
        tak(y - 1, z, x),
        tak(z - 1, x, y)
    );
}

int main() {
    int result = tak(30, 20, 10);
    printf("Result: %d\n", result);
    return result;
}
