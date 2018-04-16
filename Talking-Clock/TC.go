package main

import (
	"fmt"
	"strconv"
	"strings"
)

var numTable = map[int]string{
	0:  "twelve",
	1:  "one",
	2:  "two",
	3:  "three",
	4:  "four",
	5:  "five",
	6:  "six",
	7:  "seven",
	8:  "eight",
	9:  "nine",
	10: "ten",
	11: "eleven",

	// below code is for minutes

	12: "twelve",
	13: "thirteen",
	14: "fourteen",
	15: "fifteen",
	16: "sixteen",
	17: "seventeen",
	18: "eighteen",
	19: "nineteen",

	20: "twenty",
	30: "thirty",
	40: "fourty",
	50: "fifty",
}

func hoursParse(input string) (string, string) {
	num, _ := strconv.Atoi(input)

	if num >= 12 {
		return numTable[num-12], "pm"
	} else {
		return numTable[num], "am"
	}
}

func minuteParse(input string) string {
	num, _ := strconv.Atoi(input)

	if num == 0 {
		return ""
	} else if num < 10 {
		return fmt.Sprintf("oh %s", numTable[num])
	} else if num <= 20 {
		return fmt.Sprintf("%s", numTable[num])
	} else if num < 60 {
		if num < 30 {
			return fmt.Sprintf("%s %s", numTable[20], numTable[num-20])
		}
		if num == 30 {
			return fmt.Sprintf("%s", numTable[30])
		}
		if num < 40 {
			return fmt.Sprintf("%s %s", numTable[30], numTable[num-30])
		}
		if num == 40 {
			return fmt.Sprintf("%s", numTable[40])
		}
		if num < 50 {
			return fmt.Sprintf("%s %s", numTable[40], numTable[num-40])
		}
		if num == 50 {
			return fmt.Sprintf("%s", numTable[50])
		}
		return fmt.Sprintf("%s %s", numTable[50], numTable[num-50])
	} else {
		panic("minutes wrong")
	}
}

func main() {
	testData := []string{
		"00:00",
		"01:30",
		"12:05",
		"14:01",
		"20:29",
		"21:00",
	}

	for _, data := range testData {
		splitString := strings.Split(data, ":")

		hours, noon := hoursParse(splitString[0])
		minutes := minuteParse(splitString[1])

		if minutes == "" {
			fmt.Printf("It's %s %s\n", hours, noon)
		} else {
			fmt.Printf("It's %s %s %s\n", hours, minutes, noon)
		}
	}

}
