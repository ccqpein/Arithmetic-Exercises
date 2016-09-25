testStr = "25525511135"


class Solution(object):

    def getNext(self, sPre, sLeft):
        preSL = [sPre, ]
        tempStore = []
        for i in range(3):
            if int(sLeft[i + 1]) < 3:
                tempStore.append(sLeft[0:i + 1])
        return [preSL + [ss] for ss in tempStore]

    def restoreIpAddresses(self, s):
        """
        :type s: str
        :rtype: List[str]
        """
        print(self.getNext("", s))

if __name__ == "__main__":
    a = Solution()
    a.restoreIpAddresses(testStr)
