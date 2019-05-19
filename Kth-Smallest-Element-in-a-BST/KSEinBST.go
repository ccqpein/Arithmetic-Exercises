package main

import (
	"fmt"
	"sort"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// func lastNode(root *TreeNode, nodes []*TreeNode) []*TreeNode {
// 	inner_nodes := append(nodes, root)
// 	if root.Left != nil {
// 		return lastNode(root.Left, inner_nodes)
// 	}
// 	// } else if root.Right != nil {
// 	// 	return lastNode(root.Right, append(nodes, root))
// 	// }

// 	return inner_nodes
// }

// func kthSmallest(root *TreeNode, k int) int {
// 	call_chain := lastNode(root, []*TreeNode{})
// 	length := len(call_chain)
// 	ind := length - 1

// 	var this *TreeNode

// 	for i := 0; i < k; {
// 		if ind >= 0 {
// 			this = call_chain[ind]

// 			if this.Right != nil {
// 				if i == k-2 {
// 					return this.Right.Val
// 				} else {
// 					i += 2
// 					ind -= 1

// 				}
// 			} else {
// 				i += 1
// 				ind -= 1
// 			}

// 			if ind < 0 && i != k {
// 				this = this.Right
// 			}
// 		} else {
// 			println(i)
// 			call_chain = lastNode(this, []*TreeNode{})
// 			length = len(call_chain)
// 			ind = length - 1
// 		}
// 	}

// 	return this.Val
// }

func getAllValue(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	result := []int{}

	result = append(result, root.Val)
	result = append(result, getAllValue(root.Left)...)
	result = append(result, getAllValue(root.Right)...)

	return result
}

func kthSmallest2(root *TreeNode, k int) int {
	temp := getAllValue(root)
	sort.Ints(temp)
	return temp[k-1]
}

func main() {
	// testcase0 := &TreeNode{
	// 	Val: 3,
	// 	Left: &TreeNode{
	// 		Val:  1,
	// 		Left: nil,
	// 		Right: &TreeNode{
	// 			Val: 2,
	// 		},
	// 	},
	// 	Right: &TreeNode{
	// 		Val: 4,
	// 	},
	// }

	//fmt.Printf("%+v", lastNode(testcase0, []*TreeNode{}))
	//fmt.Printf("%+v\n", kthSmallest(testcase0, 1)) //=>1
	//fmt.Printf("%+v\n", kthSmallest(testcase0, 2)) //=>2
	//fmt.Printf("%+v\n", kthSmallest(testcase0, 3)) //=>3
	//fmt.Printf("%+v\n", kthSmallest(testcase0, 4)) //=>4

	testcase1 := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 3,
			Left: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 1,
				},
				Right: nil,
			},
			Right: &TreeNode{
				Val: 4,
			},
		},
		Right: &TreeNode{
			Val: 6,
			Right: &TreeNode{
				Val: 8,
			},
		},
	}

	fmt.Printf("%+v\n", kthSmallest2(testcase1, 3)) //=> 3
	fmt.Printf("%+v\n", kthSmallest2(testcase1, 6)) //=> 6
	fmt.Printf("%+v\n", kthSmallest2(testcase1, 4)) //=> 4
	fmt.Printf("%+v\n", kthSmallest2(testcase1, 7)) //=> 7

	testcase2 := &TreeNode{
		Val: 1,
	}

	//fmt.Printf("Call chain:%+v\n", lastNode(testcase2, []*TreeNode{}))
	fmt.Printf("%+v\n", kthSmallest2(testcase2, 1)) //=> 1

}
