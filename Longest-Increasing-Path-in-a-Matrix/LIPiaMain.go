package main

import (
	. "./LIPiaM"
)

func main() {
	test1 := Matrix{
		{9, 9, 4},
		{6, 6, 8},
		{2, 1, 1}}

	Flow(test1)
	FlowC(&test1)
}
