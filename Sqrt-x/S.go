package main

import (
	. "fmt"
)

func MySqrt(x int) int {
	switch x {
	case 1, 0:
		return x
	}

	re1 := 0
	re2 := 1
	for re2 != re1 {
		re1 = re2
		re2 = (re1 + x/re1) / 2
	}

	return int(re2)
}

func main() {
	Println(MySqrt(0))
	Println(MySqrt(1))
	Println(MySqrt(125348))
}
