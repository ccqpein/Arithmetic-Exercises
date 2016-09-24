testStr = "25525511135"


def restoreIpAddresses(s):
    strPool = ["", ]
    for i in range(4):
        for ss in strPool:
            index = 0
            if len(ss) != 0:
                index = len(ss) - i

            if ss[index] <= 2:
