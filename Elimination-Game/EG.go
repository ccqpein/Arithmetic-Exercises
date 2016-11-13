package main

import (
	"fmt"
)

func lastRemaining(n int) int {
	var l []int
	for i := 1; i <= n; i++ {
		l = append(l, i)
	}

	for len(l) != 1 {
		tempL := []int{}
		tempL2 := []int{}
		for i, e := range l {
			if i%2 != 0 {
				tempL = append(tempL, e)
			}
		}
		fmt.Println(tempL)
		if len(tempL) == 1 {
			l = tempL
			break
		}

		if len(tempL)%2 == 0 {
			for i, e := range tempL {
				if i%2 == 0 {
					tempL2 = append(tempL2, e)
				}
			}
		} else {
			for i, e := range tempL {
				if i%2 != 0 {
					tempL2 = append(tempL2, e)
				}
			}
		}
		fmt.Println("here")
		fmt.Println(tempL2)
		l = tempL2
	}

	return l[0]
}

func main() {
	n := 9

	fmt.Println(lastRemaining(n))
}
