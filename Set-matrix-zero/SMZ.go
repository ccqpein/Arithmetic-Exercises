package main

import "fmt"

func setZeroes(matrix [][]int) {
	lengthM := len(matrix)
	colLen := len(matrix[0])

	var zeroRow = make([]int, colLen)
	var colZeroIndex []int

	for i := 0; i < lengthM; i++ {
		testI := 0
		for colIndex, ele := range matrix[i] {
			if ele == 0 {
				colZeroIndex = append(colZeroIndex, colIndex)
				testI += 1
			}
		}
		if testI != 0 {
			for rei := 0; rei < i; rei++ {
				for _, ii := range colZeroIndex {
					matrix[rei][ii] = 0
				}
			}
			matrix[i] = zeroRow
		} else {
			for _, ii := range colZeroIndex {
				matrix[i][ii] = 0
			}
		}

	}
	fmt.Print(matrix)
}

func main() {
	matrix := [][]int{
		[]int{1, 2, 0, 3, 4, 5},
		[]int{0, 4, 5, 6, 7, 8},
		[]int{1, 2, 3, 4, 5, 6},
	}
	setZeroes(matrix)

	matrix2 := [][]int{
		[]int{1, 2, 3, 3, 4, 5},
		[]int{0, 4, 5, 6, 7, 8},
		[]int{1, 2, 3, 4, 5, 6},
	}
	setZeroes(matrix2)
}
