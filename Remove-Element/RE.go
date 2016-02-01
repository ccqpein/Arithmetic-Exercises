package main

import (
	. "fmt"
	"os"
	. "strconv"
)

var args = os.Args[1:]

func removeEle(num []int, ele int) []int {
	var new []int
	for _, e := range num {
		if e != ele {
			new = append(new, e)
		}
	}

	return new
}

func main() {
	var numList []int
	for _, i := range args[0] {
		numList = append(numList, int(i-'0'))
	}

	bb, _ := Atoi(args[1])
	aa := removeEle(numList, bb)
	Println(aa)

}
