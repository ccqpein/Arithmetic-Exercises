package LIPiaM

import (
	"errors"
	. "fmt"
)

// Define some types
type Matrix [][]int

type coord [2]int

type aroundRe struct {
	num       int // How many larger point around this point
	selfCoord coord
	coordL    []coord // Results coordinate
	longestP  int     // The longest path number (included this point) for store the results have done before
}

type mapTable map[coord]*aroundRe // Use *aroudRe because cannot change the value in struct which in map

func maxSlice(l []int) (int, int) { // Little useful function
	max := l[0]
	maxInd := 0
	for i, value := range l {
		if value > max {
			max = value
			maxInd = i
		}
	}
	return maxInd, max
}

func PrintMM(m mapTable) { // Little function to print the *struct in map
	for i, v := range m {
		Print(i)
		Print(":")
		Println(*v)
	}
}

func getVal(m *Matrix, c coord) (int, error) { // Get around point num
	v := *m

	if (c[0] < 0 || c[0] >= len(v)) || (c[1] >= len(v[0]) || c[1] < 0) {
		return 0, errors.New("no value")
	} else {
		return v[c[0]][c[1]], nil
	}
}
