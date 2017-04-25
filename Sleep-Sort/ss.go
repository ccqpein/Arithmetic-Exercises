package main

import (
	. "fmt"
	"time"
)

func sleepNum(n int, cc chan int) {
	time.Sleep(time.Duration(n) * time.Second)
	cc <- n

}

func main() {
	cc := make(chan int)

	testData := []int{1, 5, 6, 4, 3, 2, 7}

	for _, i := range testData {
		go sleepNum(i, cc)
	}

	for i := 0; i < len(testData); i++ {
		Println(<-cc)
	}

}
