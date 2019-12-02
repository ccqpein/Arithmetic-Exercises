package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func distributeCoins(root *TreeNode) int {
	total := 0
	dfs(&total, root)
	return total
}

func dfs(total *int, root *TreeNode) int {
	if root == nil {
		return 0
	}

	L := dfs(total, root.Left)
	R := dfs(total, root.Right)
	if L <= 0 {
		*total += -L
	} else {
		*total += L
	}
	if R <= 0 {
		*total += -R
	} else {
		*total += R
	}

	return root.Val + L + R - 1
}

func main() {}
