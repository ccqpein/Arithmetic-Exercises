package main

import "fmt"

func twoSum(nums []int, target int) []int {
	length := len(nums)

	for i := 0; i < length; i++ {
		for j := i + 1; j < length; j++ {
			if nums[i]+nums[j] == target {
				return []int{i, j}
			}
		}
	}
	return nil
}

func main() {

	testcase0 := []int{2, 7, 11, 15}
	testtarget0 := 9

	fmt.Println(twoSum(testcase0, testtarget0))

	testcase1 := []int{0, 4, 3, 0}
	testtarget1 := 0

	fmt.Println(twoSum(testcase1, testtarget1))

	testcase2 := []int{-3, 4, 3, 90}
	testtarget2 := 0

	fmt.Println(twoSum(testcase2, testtarget2))
}
