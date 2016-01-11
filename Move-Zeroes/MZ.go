package main

import (
	"fmt"
	"os"
)

func MoveZeros(nums ...int) []int {
	//nums := []int{1, 2, 3}
	x := 1
	for i := 0; i < len(nums); i++ {
		if nums[x] == 0 {
			nums = append(nums[:x], nums[x+1:len(nums)]...)
			nums = append(nums, 0)
		} else {
			x++
		}

	}
	fmt.Print(nums)
	return nums
}
func main() {
	MoveZeros(os.Args[1:])
}
