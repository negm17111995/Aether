// C++ Heavy Benchmark - Tak Recursion
#include <iostream>

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
    std::cout << "Result: " << result << std::endl;
    return result;
}
