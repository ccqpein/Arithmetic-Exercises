package main

import (
	"fmt"
)

type numInput int

func (num numInput) GeneTabel() [][]int {
	var squaresList []int
	for i := 1; i < int(num); i++ {
		squaresList = append(squaresList, i*i)
	}

	var squaresTable [][]int
	for r, s := range squaresList {
		for c, ss := range squaresList {
			squaresTable[r][c] = s + ss
		}
	}
	return squaresTable

}

func main() {
	var nn numInput = 5
	fmt.Println("new version")
	fmt.Println(nn.GeneTabel())
}
