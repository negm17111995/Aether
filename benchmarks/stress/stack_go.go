// STRESS TEST - Stack Depth (Go)
package main

import "fmt"

var depth int64 = 0

func recurse() {
	depth++
	if depth%100000 == 0 {
		fmt.Printf("Depth: %d\n", depth)
	}
	recurse()
}

func main() {
	fmt.Println("Go Stack Depth Stress Test")
	recurse()
}
