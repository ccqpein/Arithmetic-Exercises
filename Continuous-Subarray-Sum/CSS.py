class Solution(object):

    def checkSubarraySum(self, nums, k):
        """
        :type nums: List[int]
        :type k: int
        :rtype: bool
        """
        if len(nums) <= 1:
            return False
        elif sum(nums) == 0:
            return True
        elif k == 0:
            return False
        elif len(nums) >= 2 * k and k > 0:
            return True

        k = abs(k)
        result = {(0, 0): nums[0]}
        for count in range(2, len(nums) + 1):
            for i in range(0, len(nums) - count + 1):
                try:
                    tempRe = result[(i, i + count - 2)] + nums[i + count - 1]
                except:
                    tempRe = sum(nums[i:i + count])
                if tempRe % k == 0:
                    return True
                result[(i, i + count - 1)] = tempRe

        return False


a = Solution()
print(a.checkSubarraySum([23, 2, 4, 6, 7], 6))  # true
print(a.checkSubarraySum([23, 2, 6, 4, 7], -6))  # true
print(a.checkSubarraySum([0, 0], -1))  # true
print(a.checkSubarraySum([1, 1], 2))  # true
print(a.checkSubarraySum([1, 2, 3], 4))  # false
print(a.checkSubarraySum([1, 2, 3, 2, 10, 2], 14))  # true
