package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func detectCycle(head *ListNode) *ListNode {
	table := map[*ListNode]struct{}{}
	for {
		if head == nil {
			return nil
		}
		if _, ok := table[head]; ok {
			return head
		}
		table[head] = struct{}{}

		head = head.Next
	}
}

func main() {}
