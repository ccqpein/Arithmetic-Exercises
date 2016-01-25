package main

import (
	"fmt"
	"os"
)

var args []string = os.Args[1:]

type Mystring struct {
	string string
}

func (s Mystring) reversed() (rs string) {
	runes := []rune(s.string)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func isAnagram(s string, t string) {
	tt := new(Mystring)
	tt.string = t
	t = tt.reversed()
	index := 0
	for _, c := range t {
		if rune(s[index]) == c {
			index = index + 1
			continue
		} else {
			fmt.Println("False")
			break
		}
	}
	//fmt.Println(index)
	if index == len(t) {
		fmt.Println("True")
	}

	//fmt.Println(s, t)
}

func main() {
	isAnagram(args[0], args[1])
}
