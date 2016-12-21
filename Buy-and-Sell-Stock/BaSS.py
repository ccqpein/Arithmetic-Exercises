test1 = []  # return 0
test2 = [2, 1, 2, 1, 0, 1, 2]  # return 2
test3 = [1]  # return 0
test4 = [7, 1, 5, 3, 6, 4]  # return 5
test5 = [2, 4, 1]  # return 2


class Solution(object):

    def maxProfit(self, prices):
        """
        :type prices: List[int]
        :rtype: int
        """
        maxT = 0
        result = 0
        try:
            minT = prices[0]
            testV = prices[1]
        except:
            return 0

        for i in prices[1:]:
            print("this {0}".format(i))
            if i - minT < 0:
                minT = i
                print("now, min is {0}".format(i))
            else:
                if i - minT > result:
                    maxT = i
                    print("now, max is {0}".format(i))
                    result = maxT - minT
                else:
                    pass

#        result = maxT - minT

        if maxT - minT < 0:
            return 0
        else:
            return result
