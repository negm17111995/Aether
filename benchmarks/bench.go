// Go Heavy Benchmark - CPU, Memory, Concurrency
package main

import (
	"fmt"
	"runtime"
	"sync"
	"time"
)

func isPrime(n int) bool {
	if n < 2 {
		return false
	}
	if n == 2 {
		return true
	}
	if n%2 == 0 {
		return false
	}
	for i := 3; i*i <= n; i += 2 {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func countPrimes(limit int) int {
	count := 0
	for i := 2; i < limit; i++ {
		if isPrime(i) {
			count++
		}
	}
	return count
}

func memoryStress(allocs int) int64 {
	var total int64 = 0
	for i := 0; i < allocs; i++ {
		data := make([]byte, 1024)
		_ = data
		total += int64(i)
	}
	return total
}

func fib(n int) int64 {
	if n <= 1 {
		return int64(n)
	}
	var a, b int64 = 0, 1
	for i := 2; i <= n; i++ {
		a, b = b, a+b
	}
	return b
}

func main() {
	runtime.GOMAXPROCS(runtime.NumCPU())
	start := time.Now()

	// CPU Test
	primes := countPrimes(100000)

	// Memory Test
	mem := memoryStress(10000)

	// Compute Test
	f := fib(40)

	// Concurrency Test: 4 goroutines
	var wg sync.WaitGroup
	for i := 0; i < 4; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			countPrimes(10000)
		}()
	}
	wg.Wait()

	elapsed := time.Since(start).Milliseconds()

	fmt.Printf("Go Results:\n")
	fmt.Printf("  Primes: %d\n", primes)
	fmt.Printf("  Fib(40): %d\n", f)
	fmt.Printf("  Mem: %d\n", mem)
	fmt.Printf("  Time: %d ms\n", elapsed)
}
