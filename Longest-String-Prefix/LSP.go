package main

import (
	. "fmt"
)

func longestPrefix(s string) int {
	var alphabet = map[rune]int{
		'a': 1,
		'b': 2,
		'c': 3,
		'd': 4,
		'e': 5,
		'f': 6,
		'g': 7,
		'h': 8,
		'i': 9,
		'j': 10,
		'k': 11,
		'l': 12,
		'm': 13,
		'n': 14,
		'o': 15,
		'p': 16,
		'q': 17,
		'r': 18,
		's': 19,
		't': 20,
		'u': 21,
		'v': 22,
		'w': 23,
		'x': 24,
		'y': 25,
		'z': 26}
	var stack = 0
	var templong = 0
	var longest = 0
	for _, c := range s {
		if ind, _ := alphabet[c]; ind >= stack {
			templong += 1
			stack = ind
		} else {
			if templong > longest {
				longest = templong
			}
			templong = 1
			stack = ind
		}
	}

	if longest == 0 && templong != 0 {
		longest = templong
	}

	return longest
}

func main() {
	Println(longestPrefix("knotty"))
	Println(longestPrefix("apple"))
	Println(longestPrefix("excel"))
}
