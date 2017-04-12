package main

import (
	. "fmt"
)

func quick_sort(l []int) []int {
	//Println(l)
	if len(l) == 0 {
		return []int{}
	}

	var head []int
	var tail []int
	var key = l[0]

	for _, v := range l[1:] {
		if v <= key {
			head = append(head, v)
		} else {
			tail = append(tail, v)
		}
	}

	result := append(quick_sort(head), key)
	result = append(result, quick_sort(tail)...)
	return result
}

func main() {
	a := []int{2, 3, 4, 5, 2, 1, 4, 5, 3}
	Println(quick_sort(a))
}
