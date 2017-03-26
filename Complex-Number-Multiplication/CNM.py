class Solution(object):

    def complexNumberMultiply(self, a, b):
        """
        :type a: str
        :type b: str
        :rtype: str
        """
        aL = a.split("+", 1)
        bL = b.split("+", 1)
        print(aL, bL)
        ins = int(aL[0]) * int(bL[0]) - int(aL[1][:-1]) * int(bL[1][:-1])
        com = int(aL[0]) * int(bL[1][:-1]) + int(bL[0]) * int(aL[1][:-1])

        if ins != "0":
            return str(ins) + "+" + str(com) + "i"
        else:
            return str(com) + "i"
