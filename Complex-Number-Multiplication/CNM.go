package main

import (
	. "fmt"
	. "strconv"
	"strings"
)

func complexNumberMultiply(a, b string) string {
	aL := strings.Split(a, "+")
	bL := strings.Split(b, "+")
	Println(aL, bL)

	a0, _ := Atoi(aL[0])
	b0, _ := Atoi(bL[0])
	a1, _ := Atoi(aL[1][:len(aL[1])-1])
	b1, _ := Atoi(bL[1][:len(bL[1])-1])

	ins := a0*b0 - a1*b1
	com := a0*b1 + b0*a1

	if ins != 0 {
		return Itoa(ins) + "+" + Itoa(com) + "i"
	} else {
		return "0" + "+" + Itoa(com) + "i"
	}

}

func main() {
	Println(complexNumberMultiply("1+1i", "1+1i"))
	Println(complexNumberMultiply("1+-1i", "1+-1i"))
}
