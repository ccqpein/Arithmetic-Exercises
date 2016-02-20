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


def pointMatrix(twoDimMatrix, rowNum, colNum):
    def _first(func):
        def _sencond():
            func
        return _sencond
    return _first


def genList(matrix, row, col, parentList):
    for changeNum in [-1, 1]:
        if findAround(matrix, row + changeNum, col) and \
           matrix[row][col] < matrix[row + changeNum][col]:
            pass
        if findAround(matrix, row, col + changeNum) and \
           matrix[row][col] < matrix[row][col + changeNum]:
               
            pass


def main(m):

    sortedList = changeDataToList(m)
    print(startIndex)
