class Solution(object):

    def nextGreaterElements(self, nums):
        """
        :type nums: List[int]
        :rtype: List[int]
        """
        stack, res = [], [-1] * len(nums)
        for i in range(len(nums)):
            #print(stack, res)
            while stack and (nums[stack[-1]] < nums[i]):
                res[stack.pop()] = nums[i]
            stack.append(i)

        # run twice because we need list be circle
        for i in range(len(nums)):
            #print(stack, res)
            while stack and (nums[stack[-1]] < nums[i]):
                res[stack.pop()] = nums[i]
            stack.append(i)
        return res

a = Solution()
print(a.nextGreaterElements([1, 2, 1]))
print(a.nextGreaterElements([5, 3, 4, 6, 7]))
