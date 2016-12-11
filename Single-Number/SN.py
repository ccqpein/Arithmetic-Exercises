class Solution(object):

    def singleNumber(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        numDict = {}

        for i in nums:
            try:
                numDict[i] += 1
            except:
                numDict[i] = 1

        for key, value in numDict.items():
            if value == 1:
                return key
