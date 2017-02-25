package RSQ

//import . "fmt"

type NumArray struct {
	nn []int
}

func Constructor(nums []int) NumArray {
	return NumArray{nn: nums}
}

func (this *NumArray) SumRange(i int, j int) int {
	result := 0
	for _, n := range this.nn[i : j+1] {
		result += n
	}
	return result
}

/*
func main() {
	obj := Constructor([]int{-2, 0, 3, -5, 2, -1})
	param_1 := obj.SumRange(0, 2)
	Println(param_1)
}
*/
