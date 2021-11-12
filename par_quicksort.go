package main

import (
	"fmt"
	"math/rand"
	"time"
)

const Length = 100000

func quickSort(nums []int, ch chan int) {
    if len(nums) == 0 {
        close(ch)
        return
    }
    if len(nums) == 1 {
        ch <- nums[0]
        close(ch)
        return
    }
    less, greater := []int{}, []int{}
    core := nums[0]
    nums = nums[1:]
    for _, v := range nums {
        if v <= core {
            less = append(less, v)
        } else {
            greater = append(greater, v)
        }
    }
    leftCh, rightCh := make(chan int, len(less)), make(chan int, len(greater))
    go quickSort(less, leftCh)
    go quickSort(greater, rightCh)

    for v := range leftCh {
        ch <- v
    }
    ch <- core
    for v := range rightCh {
        ch <- v
    }
    close(ch)
    return
}

func main() {
    nums := []int{}
    ch := make(chan int, Length)  //Cushion Channel
    //var ch chan int
    fmt.Println(ch == nil)
    for i := 0; i < Length; i++ {
        nums = append(nums, rand.Int()) //Initialization array, length Length = 10000
    }
    now := time.Now()
	quickSort(nums, ch)
    fmt.Println("Array length:", Length, "  time consuming:", time.Since(now))
}