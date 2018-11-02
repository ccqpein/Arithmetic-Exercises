import math


class Solution:
    def judgeSquareSum(self, c):
        """
        :type c: int
        :rtype: bool
        """

        if c == 0:
            return True
        elif c < 0:
            return False

        for i in range(0, int(math.sqrt(c))+1):
            tempI = i ** 2
            try:
                tempRest = math.sqrt(c-tempI)
            except:
                continue
            if int(tempRest) == tempRest:
                return True

        return False


a = Solution()
a.judgeSquareSum(5)
