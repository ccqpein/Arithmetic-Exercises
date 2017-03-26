from math import sqrt


class Solution(object):

    def checkPerfectNumber(self, num):
        """
        :type num: int
        :rtype: bool
        """
        result = 0
        if num < 0:
            return False
        elif num == 0:
            return False
        elif num == 1:
            return False

        for i in range(1, int(sqrt(num)) + 1):
            if num % i == 0:
                print(i, num / i)
                result += i + num / i
        if result - num == num:
            return True
        else:
            return False
