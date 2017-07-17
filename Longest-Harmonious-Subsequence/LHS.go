package main

import (
	. "fmt"
	"sort"
)

func findLHS(nums []int) int {
	dic := map[int]int{}
	for _, v := range nums {
		if _, ok := dic[v]; ok {
			dic[v] += 1
		} else {
			dic[v] = 1
		}
	}

	sortkey := []int{}
	for k := range dic {
		sortkey = append(sortkey, k)
	}
	sort.Ints(sortkey)

	result := 0
	for ind := 1; ind < len(sortkey); ind++ {
		if temp := dic[sortkey[ind]] + dic[sortkey[ind-1]]; sortkey[ind]-sortkey[ind-1] == 1 &&
			temp > result {
			result = temp
		}
	}

	return result
}

func main() {
	Println(findLHS([]int{1, 3, 2, 2, 5, 2, 3, 7}))

}
