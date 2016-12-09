package main

import (
	. "fmt"
	"math"
)

func SoE(n int) []int {
	endN := math.Sqrt(float64(n))
	endN = math.Ceil(endN)

	numL := func(num int) []int {
		var reL []int
		for i := 2; i <= num; i++ {
			reL = append(reL, i)
		}
		return reL
	}(n)

	for i := 2; i < int(endN); i++ {
		var tempL []int
		for ii := 0; ii < len(numL); ii++ {
			if (numL[ii] == i) || (math.Mod(float64(numL[ii]), float64(i)) != 0) {
				tempL = append(tempL, numL[ii])
			}
		}
		numL = tempL
	}

	return numL
}

func main() {
	Println(SoE(50))
}
