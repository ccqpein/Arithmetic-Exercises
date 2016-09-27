testStr = "25525511135"
testStr2 = "0000"
testStr3 = "010010"
# answer 3: ["0.10.0.10","0.100.1.0"]
testStr4 = "101023"
# answer 4: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]


class Solution(object):

    def getNext(self, sLeft):
        tempStore = []
        for i in range(3):
            try:
                # if int(sLeft[i + 1]) < 3:
                tempStore.append(sLeft[0:i + 1])
            except:
                continue
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

        reList = []
        for l in listPool:
            if l[-1] == "":
                continue
            thisBool = True
            for ii in l:
                if int(ii) > 255 or \
                   len(ii) > 3 or \
                   (len(ii) > 1 and ii[0] == "0"):
                    thisBool = False
            if thisBool is True:
                # print(l)
                reList.append(self.parseList(l))

        return reList


if __name__ == "__main__":
    a = Solution()
    a.restoreIpAddresses(testStr)
