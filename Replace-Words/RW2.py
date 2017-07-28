# save memory


class Solution(object):

    def replaceWords(self, dict, sentence):
        sentlist = sentence.split(" ")
        for i in dict:
            for m, j in enumerate(sentlist):
                if i == j[:len(i)]:
                    sentlist[m] = i
        return " ".join(sentlist)
