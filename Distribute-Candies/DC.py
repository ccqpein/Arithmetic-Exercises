class Solution:

    def distributeCandies(self, candies):
        """
        :type candies: List[int]
        :rtype: int
        """
        resultDic = {candies[0]: 1}
        num = len(candies)
        for i in candies[1:]:
            try:
                resultDic[i] += 1
            except:
                resultDic[i] = 1

        kindNum = len(resultDic.keys())
        if kindNum > num / 2:
            return int(num / 2)
        else:
            return kindNum
