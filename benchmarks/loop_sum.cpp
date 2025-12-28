#include <iostream>

long long loop_sum(long long n) {
    long long sum = 0;
    for (long long i = 0; i < n; i++) {
        sum += i;
    }
    return sum;
}

int main() {
    long long res = loop_sum(100000000);
    return (int)(res % 256);
}
