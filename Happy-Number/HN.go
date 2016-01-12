package main

import (
	"fmt"
	"os"
	//"strconv"
	//"math"
	//"reflect"
)

var args = os.Args[1]

func IsHappy(n string) []int {
	intNum := []int{}
	for _, i := range n {
		fmt.Println(int(i - '0'))
		intNum = append(intNum, int(i-'0')*int(i-'0'))
		fmt.Println(intNum)
	}

	newInt := 

	return intNum
}

func main() {
	fmt.Print(IsHappy(args))
}
