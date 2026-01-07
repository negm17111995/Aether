// C++ HEAVY BENCHMARK
#include <iostream>
#include <chrono>
#include <vector>
#include <cstring>

using namespace std;
using namespace chrono;

// TEST 1: FIBONACCI
long long fib(int n) {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

long bench_fib() {
    auto start = high_resolution_clock::now();
    long long result = fib(40);
    auto end = high_resolution_clock::now();
    auto elapsed = duration_cast<milliseconds>(end - start).count();
    cout << "Fibonacci(40) = " << result << " in " << elapsed << "ms" << endl;
    return elapsed;
}

// TEST 2: MEMORY ALLOCATION
long bench_alloc(int count) {
    auto start = high_resolution_clock::now();
    vector<vector<char>> vecs;
    vecs.reserve(count);
    for (int i = 0; i < count; i++) {
        vecs.push_back(vector<char>(1024, i % 256));
    }
    auto end = high_resolution_clock::now();
    auto elapsed = duration_cast<milliseconds>(end - start).count();
    cout << "Allocated " << count << " x 1KB blocks in " << elapsed << "ms" << endl;
    return elapsed;
}

// TEST 3: CONCURRENT SIMULATION
long process_request(int id) {
    long sum = 0;
    for (int i = 0; i < 10000; i++) sum += i;
    return sum;
}

long bench_concurrent(int requests) {
    auto start = high_resolution_clock::now();
    int completed = 0;
    for (int i = 0; i < requests; i++) {
        if (process_request(i) > 0) completed++;
    }
    auto end = high_resolution_clock::now();
    auto elapsed = duration_cast<milliseconds>(end - start).count();
    cout << "Processed " << completed << " requests in " << elapsed << "ms" << endl;
    return elapsed;
}

// TEST 4: PRIME SIEVE
int sieve(int limit) {
    vector<bool> is_prime(limit, true);
    is_prime[0] = is_prime[1] = false;
    for (int i = 2; i * i < limit; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j < limit; j += i) {
                is_prime[j] = false;
            }
        }
    }
    int count = 0;
    for (bool p : is_prime) if (p) count++;
    return count;
}

long bench_sieve() {
    auto start = high_resolution_clock::now();
    int primes = sieve(10000000);
    auto end = high_resolution_clock::now();
    auto elapsed = duration_cast<milliseconds>(end - start).count();
    cout << "Found " << primes << " primes under 10M in " << elapsed << "ms" << endl;
    return elapsed;
}

// TEST 5: VECTOR OPERATIONS
long bench_vector(int size) {
    auto start = high_resolution_clock::now();
    vector<long> a(size), b(size), c(size);
    for (int i = 0; i < size; i++) {
        a[i] = i;
        b[i] = i * 2;
    }
    for (int i = 0; i < size; i++) {
        c[i] = a[i] + b[i];
    }
    auto end = high_resolution_clock::now();
    auto elapsed = duration_cast<milliseconds>(end - start).count();
    cout << "Vector add of " << size << " elements in " << elapsed << "ms" << endl;
    return elapsed;
}

int main() {
    cout << "=== C++ HEAVY BENCHMARK ===" << endl << endl;
    
    long total = 0;
    cout << "TEST 1: Fibonacci (Recursive)" << endl;
    total += bench_fib();
    
    cout << "\nTEST 2: Memory Allocation (1M blocks)" << endl;
    total += bench_alloc(1000000);
    
    cout << "\nTEST 3: Concurrent Requests (100K)" << endl;
    total += bench_concurrent(100000);
    
    cout << "\nTEST 4: Prime Sieve (10M)" << endl;
    total += bench_sieve();
    
    cout << "\nTEST 5: Vector Operations (10M)" << endl;
    total += bench_vector(10000000);
    
    cout << "\n=== TOTAL: " << total << "ms ===" << endl;
    return 0;
}
