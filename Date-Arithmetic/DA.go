package main

import (
	"fmt"
)

type Date struct {
	d int
	m int
	y int
}

var monthDay = []int{31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}
var monthDayL = []int{31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31}

const yzero int = 2000

func leepYear(y int) []int {
	if (y%4 == 0) && (y%400 == 0 || y%100 != 0) {
		return monthDayL
	} else {
		return monthDay
	}
}

func addDate(days int, input *Date) {
	daysLeft := days + input.d

	for daysLeft > 0 {
		monthTable := leepYear(input.y)
		//fmt.Println(monthTable)

		if daysLeft > monthTable[input.m-1] {
			daysLeft = daysLeft - monthTable[input.m-1]
			input.m += 1
			input.d = 1

			if input.m > 12 {
				input.y += 1
				input.m -= 12
			}
		} else {
			input.d = daysLeft
			daysLeft = 0
		}

	}

}

func reduceDate(days int, input *Date) {
	daysLeft := input.d - days

	for daysLeft <= 0 {
		monthTable := leepYear(input.y)

		input.m -= 1
		if input.m < 1 {
			input.y -= 1
			input.m = 12
		}

		daysLeft += monthTable[input.m-1]
	}

	input.d = daysLeft
}

func main() {
	testa := Date{d: 25, m: 2, y: 2004}
	testb := Date{d: 25, m: 12, y: 2004}

	addDate(370, &testa)
	addDate(7, &testb)

	reduceDate(370, &testa)
	reduceDate(370, &testa)

	fmt.Println(testa)
	fmt.Println(testb)
}
