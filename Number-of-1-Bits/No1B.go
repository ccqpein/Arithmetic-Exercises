package main

import (
	"fmt"
	"os"
	"strconv"
)

var args []string = os.Args[1:]

func ham(n int) int {
	oneSet := 0
	for true {
		if n == 1 {
			oneSet = oneSet + 1
			break
		} else if n%2 == 1 {
			oneSet = oneSet + 1
			n = (n - 1) / 2
		} else {
			n = n / 2
		}
	}
	return oneSet
}

func main() {
	num, _ := strconv.Atoi(args[0])
	fmt.Println(ham(num))

}
