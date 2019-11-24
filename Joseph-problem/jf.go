package main

import "fmt"

func jf(k int) int {
	if inner(1, k) {
		return 1
	}

	for i := 1; ; i++ {
		if inner(i*(1+k), k) {
			return i * (1 + k)
		}
	}
}

func inner(m, k int) bool {
	flex_m := m
	sub := 0

	left := 2 * k

	for left > k {
		if (flex_m % left) == 0 {
			sub = 0
		} else if (flex_m % left) <= k {
			return false
		} else {
			sub = left - (flex_m % left)
		}
		flex_m = m - sub
		left -= 1
	}
	return true
}

func main() {
	fmt.Println(jf(15))
}
