package main

import (
	"fmt"
	"os"
)

var args []string = os.Args[1:]

func splitString(s string) []string {
	var returnArray []string
	for i := 0; i < len(s); i++ {
		returnArray = append(returnArray, string(s[i]))
	}
	return returnArray
}

func RomanToInt(s string) int {
	dicOfLetters := map[string]int{
		"I": 1,
		"V": 5,
		"X": 10,
		"D": 500,
		"L": 50,
		"C": 100,
		"M": 1000,
	}

	charList := splitString(s)
	sumNumber := 0
	char2, char1 := 0, 0
	for i := 0; i < len(charList); i++ {
		char2 = dicOfLetters[charList[i]]
		sumNumber = sumNumber + char2
		switch {
		case (char2 == 5 || char2 == 10) && char1 == 1:
			sumNumber = sumNumber - 2
		case (char2 == 1000 || char2 == 500) && char1 == 100:
			sumNumber = sumNumber - 200
		case (char2 == 100 || char2 == 50) && char1 == 10:
			sumNumber = sumNumber - 20
		}
		char1 = char2
	}
	return sumNumber
}

func main() {
	//fmt.Println(splitString("XIIX"))
	fmt.Println(RomanToInt(args[0]))
}
