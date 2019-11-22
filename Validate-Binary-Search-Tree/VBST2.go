package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
	b, _, _ := inner(root)

	return b
}

func inner(root *TreeNode) (bool, *int, *int) {
	if root == nil {
		return true, nil, nil
	}

	if root.Left == nil && root.Right == nil {
		return true, &root.Val, &root.Val
	}

	var min *int = nil
	if root.Left != nil {
		b, mininner, maxinner := inner(root.Left)
		if !b {
			return false, nil, nil
		}
		if *maxinner >= root.Val {
			return false, nil, nil
		}
		min = mininner
	} else {
		min = &root.Val
	}

	var max *int = nil
	if root.Right != nil {
		b, mininner, maxinner := inner(root.Right)
		if !b {
			return false, nil, nil
		}
		if *mininner <= root.Val {
			return false, nil, nil
		}
		max = maxinner
	} else {
		max = &root.Val
	}

	return true, min, max
}

func main() {}
