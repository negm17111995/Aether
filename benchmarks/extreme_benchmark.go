package main

import "fmt"

func fib(n int64) int64 {
	if n <= 1 {
		return n
	}
	return fib(n-1) + fib(n-2)
}

func bench_fib() int64 {
	return fib(42)
}

func bench_alloc() int64 {
	count := int64(10000000)
	var sum int64 = 0
	for i := int64(0); i < count; i++ {
		// Go GC will run eventually, but we act dumb
		ptr := new(int64)
		*ptr = i
		sum += *ptr
	}
	return sum
}

func process_request(id int64) int64 {
	var acc int64 = 0
	for i := int64(0); i < 1000; i++ {
		acc += i * id
	}
	return acc
}

func bench_requests() int64 {
	// Sequential to match others fairness
	total := int64(1000000)
	var completed int64 = 0
	for i := int64(0); i < total; i++ {
		if process_request(i) > 0 {
			completed++
		}
	}
	return completed
}

func bench_sieve() int64 {
	limit := 50000000
	isPrime := make([]bool, limit)
	for i := range isPrime {
		isPrime[i] = true
	}

	for i := 2; i*i < limit; i++ {
		if isPrime[i] {
			for j := i * i; j < limit; j += i {
				isPrime[j] = false
			}
		}
	}

	var count int64 = 0
	for i := 2; i < limit; i++ {
		if isPrime[i] {
			count++
		}
	}
	return count
}

func main() {
	f := bench_fib()
	a := bench_alloc()
	r := bench_requests()
	s := bench_sieve()
	fmt.Printf("%d\n", f+a+r+s)
}
