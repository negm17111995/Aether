// Go HEAVY BENCHMARK
package main

import (
	"fmt"
	"time"
)

// ============================================================================
// TEST 1: FIBONACCI
// ============================================================================

func fib(n int64) int64 {
	if n <= 1 {
		return n
	}
	return fib(n-1) + fib(n-2)
}

func benchFib() int64 {
	start := time.Now()
	result := fib(40)
	elapsed := time.Since(start).Milliseconds()
	fmt.Printf("Fibonacci(40) = %d in %dms\n", result, elapsed)
	return elapsed
}

// ============================================================================
// TEST 2: MEMORY ALLOCATION
// ============================================================================

func benchAlloc(count int) int64 {
	start := time.Now()
	slices := make([][]byte, count)
	for i := 0; i < count; i++ {
		slices[i] = make([]byte, 1024)
		slices[i][0] = byte(i % 256)
	}
	elapsed := time.Since(start).Milliseconds()
	fmt.Printf("Allocated %d x 1KB blocks in %dms\n", count, elapsed)
	return elapsed
}

// ============================================================================
// TEST 3: CONCURRENT SIMULATION
// ============================================================================

func processRequest(id int) int64 {
	var sum int64 = 0
	for i := 0; i < 10000; i++ {
		sum += int64(i)
	}
	return sum
}

func benchConcurrent(requests int) int64 {
	start := time.Now()
	completed := 0

	for i := 0; i < requests; i++ {
		result := processRequest(i)
		if result > 0 {
			completed++
		}
	}

	elapsed := time.Since(start).Milliseconds()
	fmt.Printf("Processed %d requests in %dms\n", completed, elapsed)
	return elapsed
}

// ============================================================================
// TEST 4: PRIME SIEVE
// ============================================================================

func sieve(limit int) int {
	isPrime := make([]bool, limit)
	for i := range isPrime {
		isPrime[i] = true
	}
	isPrime[0] = false
	isPrime[1] = false

	for i := 2; i*i < limit; i++ {
		if isPrime[i] {
			for j := i * i; j < limit; j += i {
				isPrime[j] = false
			}
		}
	}

	count := 0
	for _, p := range isPrime {
		if p {
			count++
		}
	}
	return count
}

func benchSieve() int64 {
	start := time.Now()
	primes := sieve(10000000)
	elapsed := time.Since(start).Milliseconds()
	fmt.Printf("Found %d primes under 10M in %dms\n", primes, elapsed)
	return elapsed
}

// ============================================================================
// TEST 5: VECTOR OPERATIONS
// ============================================================================

func benchVector(size int) int64 {
	start := time.Now()

	a := make([]int64, size)
	b := make([]int64, size)
	c := make([]int64, size)

	for i := 0; i < size; i++ {
		a[i] = int64(i)
		b[i] = int64(i * 2)
	}

	for i := 0; i < size; i++ {
		c[i] = a[i] + b[i]
	}

	elapsed := time.Since(start).Milliseconds()
	fmt.Printf("Vector add of %d elements in %dms\n", size, elapsed)
	_ = c // prevent optimization
	return elapsed
}

// ============================================================================
// MAIN
// ============================================================================

func main() {
	fmt.Println("=== GO HEAVY BENCHMARK ===\n")

	fmt.Println("TEST 1: Fibonacci (Recursive)")
	benchFib()

	fmt.Println("\nTEST 2: Memory Allocation (1M blocks)")
	benchAlloc(1000000)

	fmt.Println("\nTEST 3: Concurrent Requests (100K)")
	benchConcurrent(100000)

	fmt.Println("\nTEST 4: Prime Sieve (10M)")
	benchSieve()

	fmt.Println("\nTEST 5: Vector Operations (10M)")
	benchVector(10000000)

	fmt.Println("\n=== BENCHMARK COMPLETE ===")
}
