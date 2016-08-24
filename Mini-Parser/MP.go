package main

import (
	"fmt"
	"strconv"
)

type NestedInteger struct {
	value   int
	NestedL *NestedInteger
}

func deserialize(s string) *NestedInteger {
	str := []byte(s)
	if str[0] != '[' {
		val, _ := strconv.Atoi(string(str))
		a := NestedInteger{value: val}
		return &a
	} else {
		ss := str[1 : len(str)-1]
		//fmt.Print("this string: ")
		//fmt.Println(string(ss))
		var inTemp []byte
		var breakInd int

	lo:
		for i, v := range ss {
			fmt.Println(string(v))
			switch string(v) {
			case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9":
				inTemp = append(inTemp, v)
			case "[":
				breakInd = i
				break lo
			}
		}

		val, _ := strconv.Atoi(string(inTemp))
		nextss := ss[breakInd:]

		a := NestedInteger{value: val, NestedL: deserialize(string(nextss))}
		return &a
	}
}

func main() {
	s1 := "324"
	s2 := "[123,[456,[789]]]"

	a := *deserialize(s1)
	b := *deserialize(s2)

	fmt.Println(a)
	fmt.Println(b)
}
