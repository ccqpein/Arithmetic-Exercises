def findTheDifference(s, t):
    """
    :type s: str
    :type t: str
    :rtype: str
    """
    sDic = {}
    tDic = {}
    for i in range(len(s)):
        try:
            sDic[s[i]] += 1
        except KeyError:
            sDic[s[i]] = 1

    for i in range(len(t)):
        try:
            tDic[t[i]] += 1
        except KeyError:
            tDic[t[i]] = 1

    for i in tDic.items():
        try:
            if i[1] != sDic[i[0]]:
                return i[0]
        except KeyError:
            return i[0]
