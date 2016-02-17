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


def pointMatrix(twoDimMatrix):
    def _first(func):
        def _sencond():
            row = 0
            for i in twoDimMatrix:
                col = 0
                for ii in i:
                    print(row)
                    print(col)
                    func()
                    col += 1
                row += 1
        return _sencond
    return _first


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

    print(startIndex)

#    for i in startIndex:
