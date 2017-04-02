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
        circle = 0

        # print(stuInd, stuNum)
        def doIndex(ind):
            print(stuInd)
            for i in range(stuNum):
                if M[ind][i] == 1 and i in stuInd:
                    stuInd.discard(i)
                    doIndex(i)

        while(stuInd):
            print(stuInd)
            this = stuInd.pop()
            doIndex(this)
            circle += 1

        return circle
