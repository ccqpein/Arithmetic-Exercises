package main

import (
	"fmt"
	"strconv"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	wg.Add(4)
	result := [36]string{}

	go func() {
		for i := 0; i < 36; i++ {
			if result[i] == "" {
				result[i] = strconv.Itoa(i + 1)
			}
		}
		wg.Done()
	}()

	go func() {
		for i := 2; i < 36; i += 3 {
			if result[i] != "fizz buzz" {
				result[i] = "fizz"
			}
		}
		wg.Done()
	}()

	go func() {
		for i := 4; i < 36; i += 5 {
			if result[i] != "fizz buzz" {
				result[i] = "buzz"
			}
		}
		wg.Done()
	}()

	go func() {
		for i := 14; i < 36; i += 15 {
			result[i] = "fizz buzz"
		}
		wg.Done()
	}()

	wg.Wait()
	fmt.Println(result)
}
