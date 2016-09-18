package main

import (
	. "./LIPiaM"
)

func main() {
	test1 := Matrix{
		{9, 9, 4},
		{6, 6, 8},
		{2, 1, 1}}
	test2 := Matrix{
		{3, 4, 5},
		{3, 2, 2},
		{2, 2, 1}}

	Flow(test1)
	Flow(test2)
}
