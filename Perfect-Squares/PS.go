package main

import (
	"fmt"
)

type numInput int

func (num numInput) GeneTabel() []int {
	var squaresList []int
	for i := 1; i < num; i++ {
		squaresList = append(squaresList, i*i)
	}
	return squaresList

}

func main() {
	fmt.Println("new version")
}
