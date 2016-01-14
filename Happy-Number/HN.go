package main

import (
	"fmt"
	"os"
	//"reflect"
	"strconv"
)

var args = os.Args[1]
var numberRemeber []int
var newInt int

func IsHappy(n string) string {
	if _, err := strconv.Atoi(n); err != nil {
		return n
	}

	intNum := []int{}
	newInt = 0
	var err string = "nil"
	for _, i := range n {
		//fmt.Println(int(i - '0'))
		intNum = append(intNum, int(i-'0')*int(i-'0'))
		//fmt.Println(intNum)
		newInt = newInt + int(i-'0')*int(i-'0')
	}
	fmt.Println(strconv.Itoa(newInt))
	for _, i := range numberRemeber {
		if newInt == i {
			err = "loop error"
		}
	}
	numberRemeber = append(numberRemeber, newInt)
	//fmt.Println(newInt, numberRemeber)

	if iint := newInt; iint == 1 {
		err = "Happy number"
	} else if iint < 10 {
		err = "Do not happy"
	} else {
		err = IsHappy(strconv.Itoa(newInt))
	}

	return err
}

func main() {
	fmt.Print(IsHappy(args))
}
