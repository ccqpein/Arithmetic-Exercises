class Solution(object):

    def findLHS(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """

        dic = {}
        for num in nums:
            try:
                dic[num] += 1
            except:
                dic[num] = 1

        keys = sorted(dic.keys())
        result = 0
        for ind in range(1, len(keys)):
            if keys[ind] - keys[ind - 1] == 1 \
               and dic[keys[ind]] + dic[keys[ind - 1]] > result:
                result = dic[keys[ind]] + dic[keys[ind - 1]]

        return result

a = Solution()
a.findLHS([1, 3, 2, 2, 5, 2, 3, 7])
