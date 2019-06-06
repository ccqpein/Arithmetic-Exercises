package main

type LRUCache struct {
	cap int

	record []int
	backet map[int]int //key -> value
}

func Constructor(capacity int) LRUCache {
	result := LRUCache{
		cap:    capacity,
		record: []int{},
		backet: map[int]int{},
	}
	return result
}

func (this *LRUCache) Get(key int) int {
	if v, ok := this.backet[key]; ok {
		for i := 0; i < len(this.record); i++ {
			if this.record[i] == key {
				first_p := this.record[:i]
				second_p := this.record[i+1:]
				this.record = append(this.record[:0], first_p...)
				this.record = append(this.record, second_p...)
				this.record = append(this.record, key)
			}
		}
		return v
	}
	return -1
}

func (this *LRUCache) Put(key int, value int) {
	if v := this.Get(key); v != -1 {
		this.backet[key] = value
		return
	}

	if len(this.record) == this.cap {
		delete(this.backet, this.record[0])
		this.record = append(this.record[:0], this.record[1:]...)
	}
	this.record = append(this.record, key)
	this.backet[key] = value
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * obj := Constructor(capacity);
 * param_1 := obj.Get(key);
 * obj.Put(key,value);
 */
func main() {
	obj := Constructor(2)
	// obj.Put(1, 1)
	// obj.Put(2, 2)
	// fmt.Printf("%+v\n", obj)

	// fmt.Printf("%+v\n", obj.Get(1))
	// obj.Put(3, 3)
	// fmt.Printf("%+v\n", obj)

	obj.Get(2)
	obj.Put(2, 6)
	obj.Get(1)
	obj.Put(1, 5)
	obj.Put(1, 2)
	obj.Get(1)
	println(obj.Get(2))
}
