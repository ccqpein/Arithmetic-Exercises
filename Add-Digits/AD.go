package main

import (
	"fmt"
	"os"
	"strconv"
)

var args []string = os.Args[1:]

func AddDigit(numStr string) string {
	numSum := 0
	var ss string
	for _, s := range numStr {
		numSum = numSum + int(s-'0')
	}

	if numSum < 10 {
		ss = strconv.Itoa(numSum)
	} else {
		ss = AddDigit(strconv.Itoa(numSum))
	}
	return ss
}

func main() {
	fmt.Println(AddDigit(args[0]))
}
