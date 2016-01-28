package main

import (
	"fmt"
)

func countSmaller(nums []int) []int {
	var nums2 []int

	for i, numb := range nums {
		timeCount := 0
		for ii := i; ii < len(nums); ii++ {
			if nums[ii] < numb {
				timeCount = timeCount + 1
			}
		}
		nums2 = append(nums2, timeCount)
	}

	return nums2

}

func main() {
	args := []int{5, 2, 6, 1}
	fmt.Println(countSmaller(args))
}
