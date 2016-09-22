package LIPiaM

import (
	. "fmt"
	"time"
)

func lookforLargerAroundC(m *Matrix, c coord, aroundReChan chan aroundRe) {
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

	aroundReChan <- this
}

func MakeLargerAroundTableC(table mapTable, around aroundRe) {
	table[around.selfCoord] = &around
}

func findLargestPathC(c coord, table mapTable) (int, []coord) {
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
		rr, cc := findLargestPathC(p, table)
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
func FlowC(m *Matrix) {
	aroundReChan := make(chan aroundRe)

	time1 := time.Now()
	go func(m *Matrix) {
		for i, r := range *m {
			for ii, _ := range r {
				c := coord{i, ii}
				lookforLargerAroundC(m, c, aroundReChan)
			}
		}
		close(aroundReChan)
	}(m)
	time2 := time.Now()
	Println(time2.Sub(time1))

	table := mapTable{}

	func(table mapTable, cc chan aroundRe) {
		for arr := range cc {
			MakeLargerAroundTableC(table, arr)
		}
	}(table, aroundReChan)

	resultVal := []int{}
	resultPath := [][]coord{}
	for i, _ := range table {
		a, b := findLargestPathC(i, table)
		resultVal = append(resultVal, a)
		resultPath = append(resultPath, b)
	}

	maxInd, v := maxSlice(resultVal)
	Println(v)
	pp := resultPath[maxInd]
	Println(pp)
	//PrintMM(table)
}

/*func main() {
	test1 := Matrix{
		{9, 9, 4},
		{6, 6, 8},
		{2, 1, 1}}
	FlowC(&test1)
}
*/
