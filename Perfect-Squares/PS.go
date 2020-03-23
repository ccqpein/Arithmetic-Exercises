package main

import (
	"fmt"
	"math"
)

// too slow
func numSquares_old(n int) int {
	if n == 0 {
		return 0
	}

	result := n
	for i := 1; i <= n; i++ {
		if i*i > n {
			break
		}
		if a := numSquares(n-i*i) + 1; a < result {
			result = a
			if result == 1 {
				break
			}
		}

	}
	return result

}

func numSquares(n int) int {
	temp := []int{}
	for i := 1; i <= n; i++ {
		if i*i == n {
			return 1
		}
		temp = append(temp, i*i)
	}

	one := temp
	flag := 1
	cache := []int{}
	for {
		for _, i := range temp {
			for _, o := range one {
				if i+o == n {
					return flag + 1
				} else if i+o < n { // in case use too many memory
					cache = append(cache, i+o)
				} else {
					break
				}
			}
		}
		temp = cache
		cache = []int{}
		flag += 1
	}
}

// not as fast as numSquares, but save a lot memory.
func numSquares_new(n int) int {
	handle := func(n int) int {
		for ; n%4 == 0; n /= 4 {

		}
		if n%8 == 7 {
			return 4
		}

		a := -1
		b := int(math.Sqrt(float64(n)))

		n -= b * b
		b += b + 1
		for a < b {
			if n < 0 {
				b -= 2
				n += b
			} else if n > 0 {
				a += 2
				n -= a
			} else {
				if a < 0 {
					return 1
				} else {
					return 2
				}
			}
		}
		return 3
	}

	sum := 0
	for i := 0; i < 10000; i++ {
		sum += handle(n)
	}
	return sum / 10000

}

func main() {
	fmt.Printf("%v\n", numSquares_new(92)) // 4
	fmt.Printf("%v", numSquares_new(279))  // 4
}
