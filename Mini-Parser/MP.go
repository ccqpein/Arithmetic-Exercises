package main

import (
	"fmt"
)

type NestedInteger struct {
	value   int
	NestedL NestedInteger
}

func (n NestedInteger) IsInteger() bool {

}

func (n NestedInteger) GetInteger() int {}

func (n *NestedInteger) SetInteger(value int) {}

func (n *NestedInteger) Add(elem NestedInteger) {}

func (n NestedInteger) GetList() []*NestedInteger {}

func deserialize(s string) *NestedInteger {
	if s[0] != "[" {
		val, _ := strconv.Atoi(s)
		a := NestedInteger{value: val}
		return &a
	} else {
		ss := s[1 : len(s)-1]
		inTemp := string("")
		for i, v := range ss {

		}
	}

}

func main(s string) {

}
