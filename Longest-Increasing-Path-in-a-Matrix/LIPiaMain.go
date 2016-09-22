package main

import (
	. "./LIPiaM"
	//	. "fmt"
	"math/rand"
	//	"time"
)

func generateMatrix(n int) Matrix {
	m := Matrix{}
	for i := 0; i < n; i++ {
		newR := []int{}
		for i := 0; i < n; i++ {
			newR = append(newR, rand.Intn(40)+1)
		}
		m = append(m, newR)
	}
	return m
}

func main() {
	/*test1 := Matrix{
		{9, 9, 4},
		{6, 6, 8},
		{2, 1, 1}}
	Print(test1)*/

	testM := generateMatrix(200)

	Flow(testM)
	FlowC(&testM)

}
