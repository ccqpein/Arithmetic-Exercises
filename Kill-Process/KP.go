package main

import (
	. "fmt"
)

func killProcess(pid, ppid []int, kill int) []int {
	var tempDic = map[int][]int{}

	for i, p := range ppid {
		if v, ok := tempDic[p]; ok {
			tempDic[p] = append(v, pid[i])
		} else {
			tempDic[p] = []int{pid[i]}
		}
	}

	//Println(tempDic)

	rest, stack := []int{}, []int{kill}

	for len(stack) > 0 {
		tempStack := []int{}
		for _, p := range stack {
			rest = append(rest, p)
			for _, pp := range tempDic[p] {
				tempStack = append(tempStack, pp)
			}
		}
		stack = tempStack
	}

	return rest
}

func main() {
	Println(killProcess([]int{1, 3, 10, 5}, []int{3, 0, 5, 3}, 5))
	Println(killProcess([]int{1, 3, 10, 5}, []int{3, 0, 5, 3}, 3))
}
