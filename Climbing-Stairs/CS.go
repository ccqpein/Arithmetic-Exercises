package main

import (
	. "fmt"
)

func Factorial(n int) int {
	var returnInt = 1
	for i := 1; i <= n; i++ {
		returnInt = returnInt * i
	}

	return returnInt
}

func main() {
	Println(Factorial(5))
}
