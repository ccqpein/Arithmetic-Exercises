package main

import "fmt"

func MoveZero(nums List)  {
	for i := nums.Front(); i != nil; i = i.Next() {
		fmt.Print(i)
	}
}

func main() {
	MoveZero([1,2,3,4,5])
}
