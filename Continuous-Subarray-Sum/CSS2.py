# this one is hige efficiency than old one


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
        storeAllRemainder = [0]
        for i in range(0, len(nums)):
            storeAllRemainder.append((storeAllRemainder[-1] + nums[i]) % k)

        print(storeAllRemainder)
        tempDic = {}
        for i in range(0, len(storeAllRemainder)):
            if storeAllRemainder[i] in tempDic:
                if i > tempDic[storeAllRemainder[i]] + 1:
                    return True
            else:
                tempDic[storeAllRemainder[i]] = i

        return False


a = Solution()
print(a.checkSubarraySum([23, 2, 4, 6, 7], 6))  # true
print(a.checkSubarraySum([23, 2, 6, 4, 7], -6))  # true
print(a.checkSubarraySum([0, 0], -1))  # true
print(a.checkSubarraySum([1, 1], 2))  # true
print(a.checkSubarraySum([1, 2, 3], 4))  # false
print(a.checkSubarraySum([1, 2, 3, 2, 10, 2], 14))  # true
