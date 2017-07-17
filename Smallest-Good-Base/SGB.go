package main

import (
	. "fmt"
	"math"
	"strconv"
)

func smallestGoodBase(n string) string {
	num, _ := strconv.Atoi(n)

	for l := math.Floor(math.Log2(float64(num))) + 1; l > 2; l-- {
		k := math.Floor(math.Pow(float64(num), 1/(l-1)))
		//Println(l, k)
		if (math.Pow(k, l)-1)/(k-1) == float64(num) {
			return strconv.Itoa(int(k))
		}

	}

	return strconv.Itoa(int(num - 1))
}

func main() {
	Println(smallestGoodBase("13"))
	Println(smallestGoodBase("4681"))
	Println(smallestGoodBase("1000000000000000000"))
}
