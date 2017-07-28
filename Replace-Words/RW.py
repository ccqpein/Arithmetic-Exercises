class Solution(object):

    def replaceWords(self, dict, sentence):
        """
        :type dict: List[str]
        :type sentence: str
        :rtype: str
        """
        tempDict = {}
        for w in dict:
            try:
                tempDict[w[0]].append(w)
            except:
                tempDict[w[0]] = [w]

        for k in tempDict.keys():
            tempDict[k].sort()

        def matchWord(a, b):  # assume a is shorter than b
            leng = len(a)
            if len(a) > len(b):
                return False
            for i in range(leng):
                if a[i] != b[i]:
                    return False
            return True

        result = " "
        for w in sentence.split(' '):
            try:
                for a in tempDict[w[0]]:
                    if matchWord(a, w):
                        result += a
                        break
            except KeyError:
                pass
            if result[-1] == ' ':
                result += w + ' '
            else:
                result += ' '
        return result[1:-1]
