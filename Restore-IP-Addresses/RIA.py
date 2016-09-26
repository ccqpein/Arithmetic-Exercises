testStr = "25525511135"


class Solution(object):

    def getNext(self, sLeft):
        tempStore = []
        for i in range(3):
            if int(sLeft[i + 1]) < 3:
                tempStore.append(sLeft[0:i + 1])
        return [[ss] + [sLeft[len(ss):]] for ss in tempStore]

    def parseList(self, l):
        reStr = l[0]
        for i in l[1:]:
            reStr += "." + i
        return reStr

    def restoreIpAddresses(self, s):
        """
        :type s: str
        :rtype: List[str]
        """
        listPool = [[s]]
        for i in range(3):
            tempList = []
            for l in listPool:
                for ll in self.getNext(l[-1]):
                    tempList.append(
                        l[:-1] + ll)
            listPool = tempList
#        print(listPool)

        reList = []
        for l in listPool:
            for ii in l:
                if int(ii) > 255:
            reList.append(self.parseList(l))

        return reList


if __name__ == "__main__":
    a = Solution()
    a.restoreIpAddresses(testStr)
