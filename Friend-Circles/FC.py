test1 = [[1, 1, 0],
         [1, 1, 0],
         [0, 0, 1]]  # 2

test2 = [[1, 1, 0],
         [1, 1, 1],
         [0, 1, 1]]

test3 = [[1, 0, 0, 1], [0, 1, 1, 0], [0, 1, 1, 1], [1, 0, 1, 1]]  # 1
test4 = [[1, 1, 0], [1, 1, 0], [0, 0, 1]]  # 2


class Solution(object):

    def findCircleNum(self, M):
        """
        :type M: List[List[int]]
        :rtype: int
        """
        stuNum = len(M)
        stuInd = {i for i in range(stuNum)}
        storeDone = set()
        circle = 0

        # print(stuInd, stuNum)

        def giveIndex(M, indexSet, stack):
            if len((indexSet)) == 0:
                return stack
            else:
                this = M[indexSet.pop()]
                for i in range(stuNum):
                    if this[i] == 1:
                        stack.add(i)
                return giveIndex(M, indexSet, stack)

        while(stuInd):
            giveIndex(M, stuInd, storeDone)
            circle += 1

            return circle
