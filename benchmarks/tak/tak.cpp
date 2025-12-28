// Tak Benchmark - C++
// Compile: clang++ -O3 -o tak_cpp tak.cpp
#include <iostream>

int tak(int x, int y, int z) {
    if (y >= x) return z;
    return tak(tak(x - 1, y, z), tak(y - 1, z, x), tak(z - 1, x, y));
}

int main() {
    int result = tak(24, 16, 8);
    std::cout << result << std::endl;
    return 0;
}
