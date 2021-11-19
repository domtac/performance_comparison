// Example from
// https://play.golang.org/p/qhLctDURThk

package main

import (
	"fmt"
	"math/rand"
	"runtime"
	"sync"
	"time"
)

func qSort(data []int) {
	if len(data) < 2 {
		return
	}
	pivot := data[0]
	left, right := 1, len(data)-1
	for right >= left {
		if data[left] <= pivot {
			left++
		} else {
			data[right], data[left] = data[left], data[right]
			right--
		}
	}
	// swap pivot into middle
	data[left-1], data[0] = data[0], data[left-1]
	qSort(data[:left-1])
	qSort(data[left:])
}

const thresh = 1e3

func qSortPar(data []int, wg *sync.WaitGroup) {
	if len(data) < 2 {
		// should have bailed to qSort by now but still
		wg.Done()
		return
	}
	pivot := data[0]
	left, right := 1, len(data)-1
	for right >= left {
		if data[left] <= pivot {
			left++
		} else {
			data[right], data[left] = data[left], data[right]
			right--
		}
	}
	// swap pivot into middle
	data[left-1], data[0] = data[0], data[left-1]

	// launch tasks for big subsorts
	if left-1 > thresh {
		wg.Add(1)
		go qSortPar(data[:left-1], wg)
	}
	if len(data)-right > thresh {
		wg.Add(1)
		go qSortPar(data[left:], wg)
	}

	// do small subsorts now
	if left-1 <= thresh {
		qSort(data[:left-1])
	}
	if len(data)-right <= thresh {
		qSort(data[left:])
	}

	// we're done
	wg.Done()
}

func quicksort(data []int) {
	wg := new(sync.WaitGroup)
	wg.Add(1)
	qSortPar(data, wg)
	wg.Wait()
}

func main() {
	runtime.GOMAXPROCS(runtime.NumCPU())
	data := make([]int, 1e5)

	for i := range data {
		data[i] = int(rand.Uint32() >> 1)
	}
	t := time.Now()
	quicksort(data)
	for i := range data[1:] {
		if data[i] > data[i+1] {
			fmt.Println("not sorted at index", i)
			panic("not sorted")
		}
	}
	fmt.Println("quicksort:", time.Now().Sub(t), "(parallel)")

	for i := range data {
		data[i] = int(rand.Uint32() >> 1)
	}
	t = time.Now()
	qSort(data)
	fmt.Println("qSort:", time.Since(t), "(serial)")
	for i := range data[1:] {
		if data[i] > data[i+1] {
			fmt.Println("not sorted at index", i)
			panic("not sorted")
		}
	}

	fmt.Println("all sorted")
}
