package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func rotateRight(head *ListNode, k int) *ListNode {
	last, len := lenList(head)
	if len == 0 {
		return nil
	}
	k = k % len
	if k == 0 {
		return head
	}
	fmt.Println(len, k)
	tail := head
	var prev *ListNode = head
	for i := 0; i < (len - k); i++ {
		prev = tail
		tail = tail.Next
	}
	prev.Next = nil
	last.Next = head

	return tail

}

func lenList(head *ListNode) (*ListNode, int) {
	len := 0
	var prev *ListNode = nil
	for {
		if head == nil {
			return prev, len
		}
		len += 1
		prev = head
		head = head.Next
	}
}

func makeLN(vals []int) *ListNode {
	var tail *ListNode = nil
	for i := len(vals) - 1; i >= 0; i-- {
		tail = &ListNode{
			Val:  vals[i],
			Next: tail,
		}
	}

	return tail
}

func printLN(t *ListNode) {
	for {
		if t == nil {
			return
		}
		fmt.Println(t.Val)
		t = t.Next
	}
}

func main() {
	//printLN(makeLN([]int{1, 2, 3, 4, 5}))

	tt := makeLN([]int{1, 2, 3, 4, 5})

	printLN(rotateRight(tt, 2))

	fmt.Println("")

	tt = makeLN([]int{1})

	printLN(rotateRight(tt, 0))
}
