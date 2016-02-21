"""
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
    return returnList"""


def findAround(matrix, row, col):
    try:
        if matrix[row][col]:
            return True
    except:
        return False

"""
def pointMatrix(twoDimMatrix, rowNum, colNum):
    def _first(func):
        def _sencond():
            func
        return _sencond
    return _first"""


def genList(matrix, row, col, parentList):
    returnDict = {}

    for changeNum in [-1, 1]:
        if findAround(matrix, row + changeNum, col) and \
           matrix[row][col] < matrix[row + changeNum][col]:
            print(matrix[row + changeNum][col])
            pathList = parentList
            print(pathList + [matrix[row + changeNum][col]])
            returnDict[(row + changeNum, col)
                       ] = pathList + [matrix[row + changeNum][col]]
        if findAround(matrix, row, col + changeNum) and \
           matrix[row][col] < matrix[row][col + changeNum]:
            print(matrix[row][col + changeNum])
            pathList = parentList
            print(pathList + [matrix[row][col + changeNum]])
            returnDict[(row, col + changeNum)
                       ] = pathList + [matrix[row][col + changeNum]]
    return returnDict


def main(m):
    pass
