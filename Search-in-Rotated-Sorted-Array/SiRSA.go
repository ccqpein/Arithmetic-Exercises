package main

import "fmt"

func search(nums []int, target int) int {
	length := len(nums)
	start := 0
	if length == 1 && nums[0] != target {
		return -1
	}

	for i := 0; i < length-1; i++ {
		if nums[i] == target {
			return i
		}
		if nums[i+1] < nums[i] {
			start = i + 1
			break
		}
	}

	fmt.Println(start)
	inner_nums := []int{}
	inner_nums = append(inner_nums, nums[start:]...)
	inner_nums = append(inner_nums, nums[:start]...)

	//fmt.Println(inner_nums)

	re := inner_func(inner_nums, target, 0)
	if re == -1 {
		return -1
	} else {
		return re + start
	}
}

func inner_func(nums []int, target int, start int) int {
	//fmt.Println(nums, start)
	if len(nums) == 0 {
		return -1
	}

	if len(nums) == 1 && nums[0] != target {
		return -1
	} else if len(nums) == 1 && nums[0] == target {
		return 0 + start
	}

	midv, ind := middleVal(nums)

	if float32(target) > midv {
		if v := inner_func(nums[ind:], target, ind); v != -1 {
			return v + start
		} else {
			return -1
		}
	} else if float32(target) < midv {
		if v := inner_func(nums[0:ind], target, 0); v != -1 {
			return v + start
		} else {
			return -1
		}
	} else if float32(target) == midv {
		if len(nums)%2 != 0 {
			return ind + start
		} else {
			return -1
		}
	} else if ind == -1 {
		return -1
	} else {
		return ind + start
	}
}

func middleVal(nums []int) (float32, int) {
	if len(nums) == 0 {
		return 0.0, -1
	}
	if a := len(nums); a%2 == 0 {
		return float32((nums[a/2] + nums[a/2-1])) / 2, a / 2
	} else {
		return float32(nums[a/2]), a / 2
	}
}

func main() {
	//testcase0 := []int{0, 1, 2, 3, 4, 5, 6, 7}
	////fmt.Println(middleVal(testcase0))
	////fmt.Println(search(testcase0, 0))

	testcase1 := []int{4, 5, 6, 7, 0, 1, 2}
	//fmt.Println(search(testcase1, 0)) //=> 4
	//fmt.Println(search(testcase1, 4)) //=> 0
	fmt.Println(search(testcase1, 3)) //=> -1

	testcase2 := []int{1, 3}
	//fmt.Println(middleVal(testcase2))
	fmt.Println(search(testcase2, 2))

	testcase3 := []int{8, 1, 2, 3, 4, 5, 6, 7}
	//fmt.Println(middleVal(testcase2))
	fmt.Println(search(testcase3, 6)) //=> 6

	testcase4 := []int{0, 1, 3, 4, 6, 8, 9}
	fmt.Println(search(testcase4, 5)) //=> -1
}
