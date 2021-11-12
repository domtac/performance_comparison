package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {

	slice := generateSlice(20001)
	start := time.Now()
	bubblesort(slice)
	elapsed := time.Since(start)
	fmt.Println("\n--- Bubble sort of ", len(slice) ,"in ---\n\n", elapsed, "\n")
    fmt.Println(slice)
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
 
func bubblesort(items []int) {
    var (
        n = len(items)
        sorted = false
    )
    for !sorted {
        swapped := false
        for i := 0; i < n-1; i++ {
            if items[i] > items[i+1] {
                items[i+1], items[i] = items[i], items[i+1]
                swapped = true
            }
        }
        if !swapped {
            sorted = true
        }
        n = n - 1
    }
}