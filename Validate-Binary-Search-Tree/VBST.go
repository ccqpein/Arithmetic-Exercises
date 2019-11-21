package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
	ll := makeAList(root)

	for i := 1; i < len(ll); i++ {
		if ll[i] <= ll[i-1] {
			return false
		}
	}

	return true
}

func makeAList(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}

	var left []int = []int{}
	if root.Left != nil {
		left = makeAList(root.Left)
	}

	var right []int = []int{}
	if root.Right != nil {
		right = makeAList(root.Right)
	}

	left = append(left, root.Val)
	return append(left, right...)
}

func main() {}
