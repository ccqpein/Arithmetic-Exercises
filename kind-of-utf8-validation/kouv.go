package main

import (
	"fmt"
)

func byteToDigit(b byte) []int {
	bits := make([]int, 8) // A byte has 8 bits

	for i := 7; i >= 0; i-- {
		mask := byte(1 << i)

		if (b & mask) != 0 {
			bits[7-i] = 1
		} else {
			bits[7-i] = 0
		}
	}
	return bits
}

func validation(bytes []byte) bool {
	if len(bytes) == 0 {
		return true
	}

	bbs := byteToDigit(bytes[0])

	// 10 at the beginning
	if bbs[0] == 1 && bbs[1] == 0 {
		return false
	}

	if bbs[0] == 0 && bbs[1] == 1 {
		return validation(bytes[1:])
	}

	if bbs[0] == 0 && bbs[1] == 0 {
		return false
	}

	byteLen := 0
	for _, b := range bbs {
		if b == 1 {
			byteLen += 1
		} else {
			break
		}
	}

	if byteLen > len(bytes) {
		return false
	}

	for _, x := range bytes[1:byteLen] {
		xx := byteToDigit(x)
		if xx[0] != 1 || xx[1] != 0 {
			return false
		}
	}

	return validation(bytes[byteLen:])

}

type testCase struct {
	bytes     []byte
	validated bool
}

func main() {
	fmt.Println(byteToDigit(0b10000000))

	testcases := []testCase{
		{
			bytes:     []byte{0b10000000},
			validated: false,
		},
		{
			bytes:     []byte{0b01000000, 0b10000000},
			validated: false,
		},
		{
			bytes:     []byte{0b01000000, 0b01000000},
			validated: true,
		},
		{
			bytes:     []byte{0b11000000},
			validated: false,
		},
		{
			bytes:     []byte{0b11000000, 0b10101111},
			validated: true,
		},
		{
			bytes:     []byte{0b01000000, 0b11100000, 0b10101111, 0b10101111},
			validated: true,
		},
		{
			bytes:     []byte{0b01000000, 0b11100000, 0b10101111, 0b10101111, 0b01111111},
			validated: true,
		},
	}

	for ind, testc := range testcases {
		fmt.Println("testcase: ", ind+1)
		fmt.Printf("should be %v, actually %v\n", testc.validated, validation(testc.bytes))

	}
}
