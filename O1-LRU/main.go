package main

import "fmt"

type LinkList struct {
	key  int
	v    int
	prev *LinkList
	next *LinkList
}

type LRUCache struct {
	cap      int
	head     *LinkList
	last     *LinkList
	innerMap map[int]*LinkList
}

func NewLRUCache(cap int) LRUCache {
	return LRUCache{
		cap:      cap,
		innerMap: map[int]*LinkList{},
	}
}

func (lc *LRUCache) Get(key int) (int, bool) {
	fmt.Printf("before get: %+v\n", lc)
	ind, ok := lc.innerMap[key]
	if !ok {
		return 0, false
	}

	if ind.prev != nil {
		ind.prev.next = ind.next
	} else {
		lc.head = ind.next
	}

	if ind.next != nil {
		ind.next.prev = ind.prev
	} else {
		lc.last = ind.prev
	}

	ind.prev = nil
	ind.next = lc.head
	lc.head.prev = ind
	lc.head = ind

	fmt.Printf("after get: %+v\n", lc)

	return ind.v, true
}

func (lc *LRUCache) Put(key, value int) int {
	if _, ok := lc.innerMap[key]; ok {
		return 0
	}

	var res int
	if len(lc.innerMap) == lc.cap {
		if lc.last != nil {
			last := lc.last
			last.prev.next = nil
			res = last.v
			lc.last = last.prev
			delete(lc.innerMap, last.key)
		} else {
			return 0
		}
	}

	newLL := &LinkList{
		key:  key,
		v:    value,
		prev: nil,
		next: lc.head,
	}

	lc.innerMap[key] = newLL
	if lc.head != nil {
		lc.head.prev = newLL
	} else {
		lc.last = newLL
	}

	lc.head = newLL

	return res
}

func (lc *LRUCache) PrintAll() {
	for key, v := range lc.innerMap {
		fmt.Printf("key: %v, value: %+v\n", key, v)
	}
}

func main() {
	lc := NewLRUCache(2)

	//[1:1]
	fmt.Printf("put 1:1 => %+v\n", lc.Put(1, 1))
	fmt.Printf("after put: %+v\n", lc)
	lc.PrintAll()
	fmt.Println("")

	//[2:2 1:1]
	fmt.Printf("put 2:2 => %+v\n", lc.Put(2, 2))
	fmt.Printf("after put: %+v\n", lc)
	lc.PrintAll()
	fmt.Println("")

	//[1:1 2:2]
	v, b := lc.Get(1)
	fmt.Printf("get 1 => %+v, %+v\n", v, b)
	lc.PrintAll()
	fmt.Println("")

	//[3:3 1:1]
	fmt.Printf("put 3:3 => %+v\n", lc.Put(3, 3))
	lc.PrintAll()
	fmt.Println("")
}
