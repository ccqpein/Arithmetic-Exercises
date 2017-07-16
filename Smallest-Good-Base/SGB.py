import math

test1 = "13"
test2 = "4681"
test3 = "1000000000000000000"


class Solution(object):

    def smallestGoodBase(self, n):
        """
        :type n: str
        :rtype: str
        """
        n = int(n)
        for l in range(int(math.log(n, 2)) + 1, 2, -1):
            k = int(math.pow(n, 1 / (l - 1)))
            print(l, k)
            if (k ** l - 1) / (k - 1) == n:
                return str(k)
        return str(n - 1)
