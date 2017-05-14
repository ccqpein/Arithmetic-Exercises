class Solution:

    def findUnsortedSubarray(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        newNums = sorted(nums)
        print(newNums, nums)

        start, stop = 0, 0
        changeFlag = False
        for i in range(len(nums)):
            if newNums[i] != nums[i] and changeFlag == False:
                changeFlag = True
                start = i
        for i in reversed(range(len(nums))):
            if newNums[i] != nums[i]:
                stop = i
                break

        print(start, stop)
        if changeFlag == False:
            return 0

        return stop - start + 1
