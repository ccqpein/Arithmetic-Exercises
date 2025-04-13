package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func largestValues(root *TreeNode) []int {
	var next []*TreeNode = []*TreeNode{root}
	var vv int
	re := []int{}
	for {
		next, vv = helper(next)
		if len(next) == 0 {
			break
		}
		re = append(re, vv)
		fmt.Println(re)
	}
	return re
}

func helper(root []*TreeNode) ([]*TreeNode, int) {
	var max int = -1 << 31
	nextRound := []*TreeNode{}
	for _, v := range root {
		if v == nil {
			continue
		}

		if v.Val > max {
			max = v.Val
		}
		nextRound = append(nextRound, v.Left, v.Right)
	}

	return nextRound, max
}

func main() {
	a := TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val:   3,
			Left:  &TreeNode{Val: 5},
			Right: &TreeNode{Val: 3},
		},
		Right: &TreeNode{
			Val:   2,
			Right: &TreeNode{Val: 9},
		},
	}

	fmt.Println(largestValues(&a))

	a = TreeNode{
		Val: 88,
		Left: &TreeNode{
			Val:   -44,
			Left:  nil,
			Right: &TreeNode{Val: 56},
		},
		Right: nil,
	}

	fmt.Println(largestValues(&a))
}
