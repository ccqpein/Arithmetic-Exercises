class Solution(object):

    def findLongestWord(self, s, d):
        """
        :type s: str
        :type d: List[str]
        :rtype: str
        """
        newD = sorted(d, key=len, reverse=True)
        tempList = []
        for word in newD:
            if len(tempList) != 0 and len(word) != len(tempList[-1]):
                return sorted(tempList)[0]
            if self.scanWord(s, word):
                tempList.append(word)
        if len(tempList) != 0:
            return sorted(tempList)[0]
        else:
            return ""

    def scanWord(self, sIn, s):
        if len(s) > len(sIn):
            return False
        i2 = 0
        for i1 in range(len(sIn)):
            if sIn[i1] == s[i2]:
                i2 += 1
            if i2 == len(s):
                return True
        return False

a = Solution()
print(a.findLongestWord("abpcplea", [
      "ale", "apple", "monkey", "plea"]))  # apple
print(a.findLongestWord("abpcplea", ["a", "b", "c"]))  # a
print(a.findLongestWord("apple", ["zxc", "vbn"]))  # ""
