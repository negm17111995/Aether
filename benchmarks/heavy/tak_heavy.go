// Go Heavy Benchmark - Tak Recursion
package main

import (
	"fmt"
	"os"
)

func tak(x, y, z int) int {
	if y >= x {
		return z
	}
	return tak(
		tak(x-1, y, z),
		tak(y-1, z, x),
		tak(z-1, x, y),
	)
}

func main() {
	result := tak(30, 20, 10)
	fmt.Printf("Result: %d\n", result)
	os.Exit(result)
}
