package RSQ

//import . "fmt"

type NumMatrix struct {
	nn [][]int
}

func Constructor2(matrix [][]int) NumMatrix {
	return NumMatrix{nn: matrix}
}

func (this *NumMatrix) SumRegion2(row1 int, col1 int, row2 int, col2 int) int {
	result := 0
	for _, row := range this.nn[row1 : row2+1] {
		for _, col := range row[col1 : col2+1] {
			result += col
		}
	}
	return result
}

/*
func main() {
	matrix := [][]int{
		{3, 0, 1, 4, 2},
		{5, 6, 3, 2, 1},
		{1, 2, 0, 1, 5},
		{4, 1, 0, 1, 7},
		{1, 0, 3, 0, 5},
	}
	obj := Constructor(matrix)
	Println(obj.SumRegion(2, 1, 4, 3)) // -> 8
	Println(obj.SumRegion(1, 1, 2, 2)) // -> 11
	Println(obj.SumRegion(1, 2, 2, 4)) // -> 12
}
*/
