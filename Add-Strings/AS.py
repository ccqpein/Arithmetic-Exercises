class Solution(object):

    def addStrings(self, num1, num2):
        """
        :type num1: str
        :type num2: str
        :rtype: str
        """
        innerDict = {'1': 1, '2': 2, '3': 3, '4': 4,
                     '5': 5, '6': 6, '7': 7, '8': 8, '9': 9, '0': 0}
        innerDict2 = {1: '1', 2: '2', 3: '3', 4: '4',
                      5: '5', 6: '6', 7: '7', 8: '8', 9: '9', 0: '0'}
        sum = 0
        for i in range(len(num1)):
            sum += (innerDict[num1[i]] * (10 ** (len(num1) - i - 1)))

        print(sum)
        for i in range(len(num2)):
            sum += (innerDict[num2[i]] * (10 ** (len(num2) - i - 1)))

        print(sum)
        reStr = ""
        while(sum >= 10):
            reStr = innerDict2[sum % 10] + reStr
            sum = sum // 10

        return innerDict2[sum] + reStr


a = Solution()

a.addStrings("0", "0")
a.addStrings("9", "99")
