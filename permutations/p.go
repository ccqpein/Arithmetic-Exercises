package main

import "fmt"

func permutations(input []int) [][]int {
	if len(input) == 1 {
		return [][]int{{input[0]}}
	}

	result := [][]int{}
	for k, v := range input {
		all_rest := []int{}
		all_rest = append(all_rest, input[:k]...)
		all_rest = append(all_rest, input[k+1:]...)
		//fmt.Printf("all_rest: %+v\n", all_rest)
		for _, vv := range permutations(all_rest) {
			result = append(result, append([]int{v}, vv...))
		}
	}

	return result
}

func main() {
	fmt.Printf("%+v\n", permutations([]int{1, 2, 3}))
}
