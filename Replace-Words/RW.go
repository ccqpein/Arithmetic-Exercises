package main

import (
	. "fmt"
	. "strings"
)

func replaceWords(dict []string, sentence string) string {
	sentlist := Split(sentence, " ")
	for _, key := range dict {
		tempList := []string{}
		for _, word := range sentlist {
			if len(key) <= len(word) && key == word[:len(key)] {
				tempList = append(tempList, key)
			} else {
				tempList = append(tempList, word)
			}
		}
		sentlist = tempList
	}
	Println(sentlist)

	result := ""
	for _, word := range sentlist {
		result += word + " "
	}

	return result
}

func main() {
	Println(replaceWords([]string{"cat", "bat", "rat"}, "the cattle was rattled by the battery"))
}
