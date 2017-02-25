package main

import (
	"./RSQ"
	. "fmt"
)

func main() {
	matrix := [][]int{
		{3, 0, 1, 4, 2},
		{5, 6, 3, 2, 1},
		{1, 2, 0, 1, 5},
		{4, 1, 0, 1, 7},
		{1, 0, 3, 0, 5},
	}
	obj := RSQ.Constructor2(matrix)
	Println(obj.SumRegion2(2, 1, 4, 3)) // -> 8
	Println(obj.SumRegion2(1, 1, 2, 2)) // -> 11
	Println(obj.SumRegion2(1, 2, 2, 4)) // -> 12
}
