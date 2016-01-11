package main

import (
	"fmt"
	"os"
	"strconv"
)

var args []string = os.Args[1:]

func ChangeStringToInt(args []string) []int {
	var nums []int
	for _, i := range args {
		i, _ := strconv.Atoi(i)
		nums = append(nums, i)
	}
	return nums
}

func MoveZeros(nums []int) []int {
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
	MoveZeros(ChangeStringToInt(args))
}
