package main

import (
	"fmt"
)

type numInput int

func (num numInput) GeneTabel() [][]int {
	var squaresList []int
	for i := 1; i <= int(num); i++ {
		squaresList = append(squaresList, i*i)
	}
	fmt.Println(squaresList)

	var squaresTable [][]int
	for _, s := range squaresList {
		var row []int
		for _, ss := range squaresList {
			row = append(row, ss+s)
		}
		squaresTable = append(squaresTable, row)
	}
	return squaresTable

}

func main() {
	var nn numInput = 5
	fmt.Println("new version")
	fmt.Println(nn.GeneTabel())
}
