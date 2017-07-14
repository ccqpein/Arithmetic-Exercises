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

	return []int{1, 2, 3}
}

func main() {
	Println(killProcess([]int{1, 3, 10, 5}, []int{3, 0, 5, 3}, 5))
}
