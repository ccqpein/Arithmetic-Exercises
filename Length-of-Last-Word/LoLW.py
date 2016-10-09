class Solution(object):

    def lengthOfLastWord(self, s):
        """
        :type s: str
        :rtype: int
        """

        sstr = self.delete_ending_space(s)
        # print(sstr)
        i = 0
        for ind in reversed(range(len(sstr))):
            if sstr[ind] == " ":
                i = ind + 1
                break
        # print(sstr[i + 1:])
        return len(sstr[i:])

    def delete_ending_space(self, s):
        i = len(s) - 1

        while(i != -1 and s[i] == " "):
            i -= 1

        return s[:i + 1]
