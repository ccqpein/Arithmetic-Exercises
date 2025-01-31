package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func swapPairs(head *ListNode) *ListNode {
	//fmt.Printf("%+v\n", *head)
	if head == nil {
		return nil
	}

	b := head.Next
	if b == nil {
		return head
	}
	c := head.Next.Next

	b.Next = head
	head.Next = swapPairs(c)
	return b
}

func main() {
	testcase0 := &ListNode{
		Val: 1,
		Next: &ListNode{
			Val: 2,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val:  4,
					Next: nil,
				},
			},
		},
	}

	testcase0 = swapPairs(testcase0)

	fmt.Printf("%+v", *testcase0)
}
