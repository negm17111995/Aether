// Go Concurrent Stress - 1000 goroutines with heavy allocation
package main

import (
	"fmt"
	"runtime"
	"sync"
	"sync/atomic"
)

const NUM_GOROUTINES = 1000
const ALLOCS_PER_GOROUTINE = 1000
const ALLOC_SIZE = 65536

func main() {
	runtime.GOMAXPROCS(runtime.NumCPU())

	fmt.Printf("Go Concurrent Stress: %d goroutines x %d allocations x %d KB\n",
		NUM_GOROUTINES, ALLOCS_PER_GOROUTINE, ALLOC_SIZE/1024)

	var totalSuccess atomic.Int64
	var failedGoroutines atomic.Int32
	var wg sync.WaitGroup

	for i := 0; i < NUM_GOROUTINES; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			defer func() {
				if r := recover(); r != nil {
					failedGoroutines.Add(1)
				}
			}()

			for j := 0; j < ALLOCS_PER_GOROUTINE; j++ {
				data := make([]byte, ALLOC_SIZE)
				data[0] = byte(id)
				totalSuccess.Add(1)
				_ = data
			}
		}(i)
	}

	wg.Wait()

	fmt.Printf("Total allocations: %d / %d\n", totalSuccess.Load(), NUM_GOROUTINES*ALLOCS_PER_GOROUTINE)
	fmt.Printf("Failed goroutines: %d / %d\n", failedGoroutines.Load(), NUM_GOROUTINES)

	if failedGoroutines.Load() > 0 {
		fmt.Printf("STATUS: FAILED (%d goroutines crashed)\n", failedGoroutines.Load())
	} else {
		fmt.Println("STATUS: PASSED")
	}
}
