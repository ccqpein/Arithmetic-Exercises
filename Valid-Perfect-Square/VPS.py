class Solution:
    def isPerfectSquare(self, num):
        """
        :type num: int
        :rtype: bool
        """
        d = 2
        init = 1
        total = 0

        while total < num:
            print(total)
            total += init
            init += d

        if total == num:
            return True

        return False


a = Solution()
