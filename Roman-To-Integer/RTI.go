package main

import (
	"fmt"
)

func splitString(s string) []string {
	var returnArray []string
	for i := 0; i < len(s); i++ {
		returnArray = append(returnArray, string(s[i]))
	}
	return returnArray
}

func RomanToInt(s string) int {
	charList := splitString(s)
	sumNumber := 0
	for i := 0; i < len(charList)-1; i++ {

	}

}

func main() {
	dicOfLetters := map[string]int{
		"I": 1,
		"V": 5,
		"X": 10,
		"D": 500,
		"L": 50,
		"C": 100,
		"M": 1000,
	}
	fmt.Println(dicOfLetters)
	fmt.Println(splitString("XIIX"))
	fmt.Println(RomanToInt("XIIX"))
}
