class Solution(object):

    def findShortestSubArray(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """

        # store_all_ele = {num:[degree,[postion]]}
        store_all_ele = {}

        stack_of_degree = [0, []]

        for ind, num in enumerate(nums):
            try:
                if len(store_all_ele[num][1]) > 1:
                    store_all_ele[num][1][1] = ind
                else:
                    store_all_ele[num][1].append(ind)
            except KeyError:
                store_all_ele[num] = [0, [ind]]
            store_all_ele[num][0] += 1

            this_degree = store_all_ele[num][0]

            if this_degree == stack_of_degree[0]:
                stack_of_degree[1].append(num)
            elif this_degree > stack_of_degree[0]:
                stack_of_degree[0] = this_degree
                stack_of_degree[1] = [num]

        # print(stack_of_degree)
        # print(store_all_ele)
        result = min([store_all_ele[x][1][-1] - store_all_ele[x]
                      [1][0] + 1 for x in stack_of_degree[1]])
        return result
