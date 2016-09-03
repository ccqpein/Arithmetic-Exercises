package main

import (
	. "fmt"
)

func MakeHashTable(s string) map[byte]int {
	var sh = map[byte]int{byte('0'): 1}
	//Because cannot get value if map is empty, give map a uneless var

	for _, c := range []byte(s) {
		i, b := sh[c]
		if !b {
			sh[c] = i + 1
		} else {
			sh[c] = 1
		}
	}
	delete(sh, byte('0'))
	return sh
}

func FindTheDifferent(s, p string) string {
	var sT = MakeHashTable(s)
	var pT = MakeHashTable(p)

	for c, v := range pT {
		i, b := sT[c]
		if i != v || b == false {
			return string(c)
		}
	}

	return ""
}

func main() {
	s1 := "abc"
	p1 := "abcd"

	s2 := "aaa"
	p2 := "abaa"

	Println(FindTheDifferent(s1, p1))
	Println(FindTheDifferent(s2, p2))
}
