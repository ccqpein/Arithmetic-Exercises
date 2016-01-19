package main

import (
	"fmt"
	"strings"
)

func ReverseWords(stringR string) []string {
	//indexSpace = make([]int, 0)
	var wordsList []string
	/**
	for i, s := range stringR {
		if s == " " {
			indexSpace = append(indexSpace, i)
		}
	}
	for i, _ := range indexSpace {
		if i < len(indexSpace)-1; indexSpace[i+1]-indexSpace[i] >= 2 {
			wordsList = append(wordsList, stringR[indexSpace[i]:indexSpace[i+1]])
		}
	}*/

	wordsList = strings.Split(stringR, " ")
	return wordsList
}

func main() {
	fmt.Print(ReverseWords("ccQ nihao"))
}
