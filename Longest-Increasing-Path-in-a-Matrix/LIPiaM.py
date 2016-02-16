def changeDataToList(matrix):
    returnList, returnList2 = [], []
    for i in matrix:
        for ii in i:
            returnList.append(ii)
    returnList = sorted(returnList)
    for i in returnList:
        if i not in returnList2:
            returnList2.append(i)
    returnList = returnList2
    return returnList


def findAround(matrix, row, col):
    try:
        if matrix[row][col]:
            return True
    except:
        return False


def main(m):
    sortedList = changeDataToList(m)
    startIndex = []
    temp1, temp2 = 0, 0

    for i in m:
        temp2 = 0
        for ii in i:
            if ii == sortedList[0]:
                startIndex.append([temp1, temp2])
            temp2 += 1
        temp1 += 1

    return startIndex
