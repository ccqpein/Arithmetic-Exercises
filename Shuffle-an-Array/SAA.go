package main

import (
	"fmt"
	"math/rand"
)

type Solution struct {
	arr map[int]int
}

func Constructor(nums []int) Solution {
	nullm := map[int]int{}
	a := Solution{arr: nullm}
	for i, e := range nums {
		a.arr[i] = e
	}

	return a
}

/** Resets the array to its original configuration and return it. */
func (this *Solution) Reset() []int {
	var a []int
	for i := 0; i < len(this.arr); i++ {
		a = append(a, this.arr[i])
	}

	return a
}

/** Returns a random shuffling of the array. */
func (this *Solution) Shuffle() []int {
	randArr := rand.Perm(len(this.arr))
	var a []int
	for _, i := range randArr {
		a = append(a, this.arr[i])
	}
	return a
}

func main() {
	nums := []int{1, 2, 3}

	obj := Constructor(nums)
	param_1 := obj.Reset()
	param_2 := obj.Shuffle()

	fmt.Println(param_1)
	fmt.Println(param_2)
}

/**
 * Your Solution object will be instantiated and called as such:
 * obj := Constructor(nums);
 * param_1 := obj.Reset();
 * param_2 := obj.Shuffle();
 */
