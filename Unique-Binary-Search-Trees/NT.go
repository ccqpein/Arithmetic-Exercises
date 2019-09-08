package main

import "fmt"

func num_trees(n int) int {
	table := map[int]int{
		0: 1,
		1: 1,
	}

	for i := 2; i <= n; i++ {
		for j := 0; j < i; j++ {
			table[i] += table[j] * table[i-j-1]
		}
	}
	fmt.Printf("%+v\n", table)
	return table[n]
}

func main() {
	println(num_trees(3))
}
