package main

import (
	"fmt"
	"os"
	"strconv"
)

var args []string = os.Args[1:]

func IsUgly(num int) bool {
	var re bool
	switch {
	case IfInOrNot(num, [4]int{1, 2, 3, 5}) == true:
		re = true
	case num == 0:
		re = false
	case num%3 == 0:
		re = IsUgly(num / 3)
	case num%5 == 0:
		re = IsUgly(num / 5)
	case num%2 == 0:
		re = IsUgly(num / 2)
	}
	return re
}

func IfInOrNot(num int, pool [4]int) bool {
	var re bool
	for _, i := range pool {
		if i == num {
			re = true
			break
		} else {
			re = false
		}
	}
	return re
}

func main() {
	num, _ := strconv.Atoi(args[0])
	fmt.Print(IsUgly(num))
}
