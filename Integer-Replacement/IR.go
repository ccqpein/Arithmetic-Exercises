package main

import "fmt"

func innerFunc(n int, count int) int {

	if n == 1 {
		return count
	}

	if n%2 == 0 {
		return innerFunc(n/2, count+1)
	} else {
		add := innerFunc(n+1, count+1)
		minus := innerFunc(n-1, count+1)

		if add <= minus {
			return add
		} else {
			return minus
		}
	}
}

func integerReplacement(n int) int {
	return innerFunc(n, 0)
}

func main() {
	fmt.Printf("%v\n", integerReplacement(8))
	fmt.Printf("%v\n", integerReplacement(7))
	fmt.Printf("%v\n", integerReplacement(1234))
	fmt.Printf("%v\n", integerReplacement(2147483647))
}
