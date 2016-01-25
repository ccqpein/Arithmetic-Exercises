package main

import (
	"fmt"
)

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
	fmt.Println(s, t)
}

func main() {
	isAnagram("aaa", "asd")
}
