package main

import (
	"errors"
	. "fmt"
)

type matrix [][]int

type coord [2]int

type aroundRe struct {
	num       int //How many results
	selfCoord coord
	coordL    []coord //Results coordinate
}

func (m *matrix) getVal(c coord) (int, error) {
	v := *m

	if (c[0] < 0 || c[0] >= len(v)) || (c[1] >= len(v[0]) || c[1] < 0) {
		return 0, errors.New("no value")
	} else {
		return v[c[0]][c[1]], nil
	}
}

func (m *matrix) lookforLargerAround(c coord) aroundRe {
	this := aroundRe{selfCoord: c}
	num := 0
	val, _ := m.getVal(c)

	var coordL []coord

	for i, v := range c {
		newC1 := c
		newC1[i] = v + 1
		val1, err := m.getVal(newC1)
		if err == nil {
			if val1 > val {
				num += 1
				coordL = append(coordL, newC1)
			}
		}

		newC2 := c
		newC2[i] = v - 1
		val2, err := m.getVal(newC2)
		if err == nil {
			if val2 > val {
				num += 1
				coordL = append(coordL, newC2)
			}
		}
	}

	this.coordL = coordL
	this.num = num

	return this
}

func (m *matrix) makeLargerAroundTable() map[coord]aroundRe { // Can use goroute futrue
	var this = map[coord]aroundRe{}
	for i, r := range *m {
		for ii, _ := range r {
			c := coord{i, ii}
			this[c] = m.lookforLargerAround(c)
		}
	}
	return this
}

func (table *map[coord]aroundRe) getLargestPath(c coord) (int, coord) {
	nextNum := *table[c].num
	number := 1

	for nextNum != 0 {

	}
}

func main() {
	test1 := matrix{
		{9, 9, 4},
		{6, 6, 8},
		{2, 1, 1}}
	//test2 := matrix{{3, 4, 5}, {3, 2, 2}, {2, 2, 1}}

	//cccc := coord{1, 1}
	//Println(test1.lookforLargerAround(cccc))
	Println(test1.makeLargerAroundTable())
}
