// C++ Heavy Benchmark - CPU, Memory, Concurrency
#include <iostream>
#include <chrono>
#include <thread>
#include <vector>
#include <cmath>

bool is_prime(long n) {
    if (n < 2) return false;
    if (n == 2) return true;
    if (n % 2 == 0) return false;
    for (long i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return false;
    }
    return true;
}

long count_primes(int limit) {
    long count = 0;
    for (int i = 2; i < limit; i++) {
        if (is_prime(i)) count++;
    }
    return count;
}

long memory_stress(int allocs) {
    long total = 0;
    for (int i = 0; i < allocs; i++) {
        auto* ptr = new char[1024];
        total += i;
        delete[] ptr;
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

void worker() {
    count_primes(10000);
}

int main() {
    auto start = std::chrono::high_resolution_clock::now();
    
    // CPU Test
    long primes = count_primes(100000);
    
    // Memory Test
    long mem = memory_stress(10000);
    
    // Compute Test
    long f = fib(40);
    
    // Concurrency Test: 4 threads
    std::vector<std::thread> threads;
    for (int i = 0; i < 4; i++) {
        threads.emplace_back(worker);
    }
    for (auto& t : threads) {
        t.join();
    }
    
    auto end = std::chrono::high_resolution_clock::now();
    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
    
    std::cout << "C++ Results:" << std::endl;
    std::cout << "  Primes: " << primes << std::endl;
    std::cout << "  Fib(40): " << f << std::endl;
    std::cout << "  Time: " << elapsed << " ms" << std::endl;
    
    return 0;
}
