#! /usr/bin/env python

testList = [1, [3, [2]], [[[5]]]]
testList2 = [1, [3, [2], [6, 7]], [[4, [5]]]]


def NL(lis):
    tempList = []
    for l in lis:
        if type(l) != list:
            tempList.append(l)
        else:
            tempList += NL(l)
    return tempList


def NL2(lis):
    lisInner = lis
    resultList = []
    while lisInner != []:
        if lisInner[0] == []:
            lisInner = lisInner[1:]
        elif type(lisInner[0]) != list:
            resultList.append(lisInner[0])
            lisInner = lisInner[1:]  # mark
        else:
            lisInner = lisInner[0] + lisInner[1:]

    return resultList


if __name__ == "__main__":
    print(NL(testList))
    print(NL(testList2))
    print(NL2(testList))
    print(NL2(testList2))
