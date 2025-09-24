package main

import "fmt"

func sum(init int, n ...int) int {
	for _, nn := range n {
		init += nn
	}

	return init
}

func countPartitions(nums []int) int {
	x := sum(nums[0], nums[1:]...)
	if x%2 != 0 {
		return 0
	}

	return len(nums) - 1
}

func main() {
	fmt.Println(countPartitions([]int{10, 10, 3, 7, 6}))
	fmt.Println(countPartitions([]int{1, 2, 2}))
	fmt.Println(countPartitions([]int{2, 4, 6, 8}))
}
