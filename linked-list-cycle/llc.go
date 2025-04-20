package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle(head *ListNode) bool {
	table := map[*ListNode]struct{}{}

	for {
		if head == nil {
			return false
		}
		if _, ok := table[head]; ok {
			return true
		}

		table[head] = struct{}{}

		head = head.Next
	}
}

func main() {}
