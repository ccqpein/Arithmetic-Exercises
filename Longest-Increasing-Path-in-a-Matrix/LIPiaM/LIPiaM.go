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

func lookforLargerAround(m *Matrix, c coord) aroundRe {
	this := aroundRe{selfCoord: c, longestP: -1}
	num := 0
	val, _ := getVal(m, c)

	var coordL []coord

	for i, v := range c {
		newC1 := c
		newC1[i] = v + 1
		val1, err := getVal(m, newC1)
		if err == nil {
			if val1 > val {
				num += 1
				coordL = append(coordL, newC1)
			}
		}

		newC2 := c
		newC2[i] = v - 1
		val2, err := getVal(m, newC2)
		if err == nil {
			if val2 > val {
				num += 1
				coordL = append(coordL, newC2)
			}
		}
	}

	this.coordL = coordL
	this.num = num

	return this
}

func makeLargerAroundTable(m *Matrix) mapTable { // Can use goroute futrue
	var this = mapTable{}
	for i, r := range *m {
		for ii, _ := range r {
			c := coord{i, ii}
			tt := lookforLargerAround(m, c)
			this[c] = &tt // Save address of struct aroundRe
		}
	}
	return this
}

func findLargestPath(c coord, table mapTable) (int, []coord) {
	this := table[c]
	reCoordList := []coord{this.selfCoord} // Struct-point can access struct directly.

	// If this point have been computed before, return the result directly.
	if this.longestP != -1 {
		return this.longestP, reCoordList
	}

	// num == 0 means this point is end point
	if this.num == 0 {
		return this.num + 1, reCoordList
	}

	// Recursion body
	tempCoordList := [][]coord{}
	tempResultInt := []int{}
	for _, p := range this.coordL {
		rr, cc := findLargestPath(p, table)
		tempResultInt = append(tempResultInt, rr)
		tempCoordList = append(tempCoordList, cc)
	}

	// Find the maximal value index
	maxInd, _ := maxSlice(tempResultInt)

	// Add return coord to result coord list
	reCoordList = append(reCoordList, tempCoordList[maxInd]...)

	this.longestP = tempResultInt[maxInd] + 1

	return this.longestP, reCoordList

}

// Flow function
func Flow(m Matrix) {
	table := makeLargerAroundTable(&m)
	resultVal := []int{}
	resultPath := [][]coord{}
	for i, _ := range table {
		a, b := findLargestPath(i, table)
		resultVal = append(resultVal, a)
		resultPath = append(resultPath, b)
	}
	maxInd, v := maxSlice(resultVal)
	Println(v)
	pp := resultPath[maxInd]
	Println(pp)
	PrintMM(table)
}

/*
func main() {
	test1 := Matrix{
		{9, 9, 4},
		{6, 6, 8},
		{2, 1, 1}}
	test2 := Matrix{
		{3, 4, 5},
		{3, 2, 2},
		{2, 2, 1}}

	Flow(test1)
	Flow(test2)

}
*/
