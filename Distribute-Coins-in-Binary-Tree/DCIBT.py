class TreeNode:
        def __init__(self, x):
            self.val = x
            self.left = None


class Solution:
    def distributeCoins(self, root: TreeNode) -> int:
        total = 0

        def dfs(node):
            if not node:
                return 0
            L, R = dfs(node.left), dfs(node.right)
            total += abs(L) + abs(R)
            return node.val + L + R - 1

        dfs(root)
        return total
