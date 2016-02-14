package main

import (
	. "fmt"
	"os"
	"strconv"
)

var args []string = os.Args[1:]

func Factorial(n int, m ...int) int {
	var returnInt = 1
	var times int

	if len(m) == 0 {
		times = n
	} else {
		times = m[0]
	}

	for i := 1; i <= times; i++ {
		returnInt = returnInt * n
		n--
	}
	return returnInt
}

func Combinatorics(n, m int, args ...string) int {
	var resultnNum int

	if args[0] == "C" || args[0] == "c" {
		resultnNum = Factorial(n, m) / Factorial(m)
	} else {
		resultnNum = Factorial(n, m)
	}

	return resultnNum
}

func genList(n int) [][]int {
	var resultnList [][]int
	for i := 0; i <= n; i++ {
		if (n-i)%2 == 0 {
			tempList := []int{i, (n - i) / 2}
			resultnList = append(resultnList, tempList)
		}
	}
	Println(resultnList)
	return resultnList
}

func main() {
	aa, _ := strconv.Atoi(args[0])
	var waysList = genList(aa)
	var allwaysNum = 0

	for _, i := range waysList {
		n := i[0] + i[1]
		m := i[0]
		allwaysNum += Combinatorics(n, m, "C")

	}
	Println(allwaysNum)
}
