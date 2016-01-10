package main

import (
	"fmt"
	"os"
)

var nums []int

func main() {
	nums := []int{1, 2, 3}
	for i := 0; i < len(nums); i++ {
		fmt.Print(nums[i])
	}
}
