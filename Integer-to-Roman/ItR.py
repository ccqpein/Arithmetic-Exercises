class Solution:
    def intToRoman(self, num):
        """
        :type num: int
        :rtype: str
        """
        table = {
            1: "I",
            4: "IV",
            5: "V",
            9: "IX",
            10: "X",
            40: "XL",
            50: "L",
            90: "XC",
            100: "C",
            400: "CD",
            500: "D",
            900: "CM",
            1000: "M",
        }

        number = num
        result = ""
        order = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1]

        while True:
            if number == 0:
                break
            if number in table:
                result += table[number]
                break
            elif number > 1000:
                for i in range(number//1000):
                    result += table[1000]
                number %= 1000
                continue
            elif number > 900:
                for i in range(number//900):
                    result += table[900]
                number %= 900
                continue
            elif number > 500:
                for i in range(number//500):
                    result += table[500]
                number %= 500
                continue
            elif number > 400:
                for i in range(number//400):
                    result += table[400]
                number %= 400
                continue
            elif number > 100:
                for i in range(number//100):
                    result += table[100]
                number %= 100
                continue
            elif number > 90:
                for i in range(number//90):
                    result += table[90]
                number %= 90
                continue
            elif number > 50:
                for i in range(number//50):
                    result += table[50]
                number %= 50
                continue
            elif number > 40:
                for i in range(number//40):
                    result += table[40]
                number %= 40
                continue
            elif number > 10:
                for i in range(number//10):
                    result += table[10]
                number %= 10
                continue
            elif number > 9:
                for i in range(number//9):
                    result += table[9]
                number %= 9
                continue
            elif number > 5:
                for i in range(number//5):
                    result += table[5]
                number %= 5
                continue
            elif number > 4:
                for i in range(number//4):
                    result += table[4]
                number %= 4
                continue
            elif number > 1:
                for i in range(number//1):
                    result += table[1]
                number %= 1
                break

        return result

    def intToRoman2(self, num):
        """
        :type num: int
        :rtype: str
        """
        table = {
            1: "I",
            4: "IV",
            5: "V",
            9: "IX",
            10: "X",
            40: "XL",
            50: "L",
            90: "XC",
            100: "C",
            400: "CD",
            500: "D",
            900: "CM",
            1000: "M",
        }

        number = num
        result = ""
        order = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1]

        while True:
            if number == 0:
                break
            if number in table:
                result += table[number]
                break

            for v in order:
                if number > v:
                    for i in range(number//v):
                        result += table[v]
                    number %= v
                    break
        return result


a = Solution()
a.intToRoman2(1)
