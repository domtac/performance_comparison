package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {

	slice := generateSlice(100001)
	start := time.Now()
	insertionsort(slice)
	elapsed := time.Since(start)
	fmt.Println("\n--- Sorting took ---\n\n", elapsed, "\n")
}

// Generates a slice of size, size filled with random numbers
func generateSlice(size int) []int {

	slice := make([]int, size, size)
	rand.Seed(time.Now().UnixNano())
	for i := 0; i < size; i++ {
		slice[i] = rand.Intn(999) - rand.Intn(999)
	}
	return slice
}
 
func insertionsort(items []int) {
    var n = len(items)
    for i := 1; i < n; i++ {
        j := i
        for j > 0 {
            if items[j-1] > items[j] {
                items[j-1], items[j] = items[j], items[j-1]
            }
            j = j - 1
        }
    }
}