package main

import "sort"

func longestConsecutive(nums []int) int {
	sort.Ints(nums)
	a := 0
	largest := 0
	var last int

	for _, i := range nums {
		if a == 0 {
			a += 1
			last = i
			continue
		}

		if i-last == 1 {
			a += 1

		} else if i-last == 0 {

		} else {
			if a >= largest {
				largest = a
			}
			a = 1
		}
		last = i

	}

	if a >= largest {
		largest = a
	}

	return largest
}

func main() {
	println(longestConsecutive([]int{100, 4, 200, 1, 3, 2})) //=> 4
	println(longestConsecutive([]int{0}))                    //=> 1
	println(longestConsecutive([]int{1, 2, 0, 1}))           //=> 3
}
