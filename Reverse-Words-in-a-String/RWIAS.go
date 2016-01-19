package main

import (
	"fmt"
	"os"
	"strings"
)

var args []string = os.Args[1:]

func ReverseWords(stringR string) []string {

	wordsList := strings.Split(stringR, " ")
	var newWordsList []string
	//fmt.Println(wordsList)

	for i := len(wordsList) - 1; i >= 0; i = i - 1 {
		newWordsList = append(newWordsList, wordsList[i])
		//fmt.Println(wordsList[i])
	}
	return newWordsList
}

func main() {
	var result string
	for _, i := range args {
		result = result + i + " "
	}
	result = strings.TrimSuffix(result, " ")
	fmt.Print(ReverseWords(result))
}
