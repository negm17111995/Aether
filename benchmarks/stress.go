// Go Extreme Stress Test
package main

import (
	"fmt"
	"os"
	"runtime"
	"strconv"
)

const ALLOC_SIZE = 1048576 // 1MB

func testAllocations(count int) int {
	runtime.GC()

	slices := make([][]byte, 0, count)
	success := 0

	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("  PANIC at allocation #%d: %v\n", success, r)
		}
	}()

	for i := 0; i < count; i++ {
		data := make([]byte, ALLOC_SIZE)
		for j := 0; j < ALLOC_SIZE; j += 4096 {
			data[j] = byte(i)
		}
		slices = append(slices, data)
		success++
	}

	return success
}

func main() {
	target := 1000
	if len(os.Args) > 1 {
		if t, err := strconv.Atoi(os.Args[1]); err == nil {
			target = t
		}
	}

	fmt.Printf("Go Stress Test: %d x 1MB allocations\n", target)

	success := testAllocations(target)

	fmt.Printf("Result: %d/%d allocations succeeded\n", success, target)
	fmt.Printf("Memory used: %d MB\n", success)

	if success < target {
		fmt.Printf("STATUS: FAILED at %d\n", success)
		os.Exit(1)
	}
	fmt.Println("STATUS: PASSED")
}
