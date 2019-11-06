package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isUnivalTree(root *TreeNode) bool {
	return isUTWithState(root, root.Val)
}

func isUTWithState(root *TreeNode, val int) bool {
	if root == nil {
		return true
	}

	if root.Val != val {
		return false
	}

	return isUTWithState(root.Left, root.Val) && isUTWithState(root.Right, root.Val)
}

func main() {
	testcase0 := TreeNode{
		1,
		&TreeNode{
			1,
			&TreeNode{
				1,
				nil,
				nil,
			},
			&TreeNode{
				2,
				nil,
				nil,
			},
		},
		&TreeNode{
			1,
			nil,
			&TreeNode{
				1,
				nil,
				nil,
			},
		},
	}

	println(isUnivalTree(&testcase0))

}
