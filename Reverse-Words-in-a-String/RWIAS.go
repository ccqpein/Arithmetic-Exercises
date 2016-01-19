package main

import (
	"fmt"
)

func ReverseWords(stringR string) []string {
	var indexSpace []int
	var wordsList []string
	for i, s := range stringR {
		if s == " " {
			indexSpace = append(indexSpace, i)
		}
	}

}
