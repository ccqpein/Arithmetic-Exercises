package main

import (
	"fmt"
)

var testS string = "BBC ABCDAB ABCDABCDABDE"
var testP string = "ABCDABD"

func makeIndexTable(pattern string) []int {
	var indexTable []int
	for i := 1; i <= len(pattern); i++ {
		thisP := pattern[:i] //not sure
		tempLen := 0
		for ii := 1; ii < i; ii++ {
			if thisP[:ii] == thisP[i-ii:] &&
				len(thisP[:ii]) > tempLen {
				tempLen = len(thisP[:ii])
			}
		}
		indexTable = append(indexTable, tempLen)
	}
	return indexTable
}

func main() {
	fmt.Print(
		makeIndexTable("abcda"),
		makeIndexTable("abcdab"))

}
