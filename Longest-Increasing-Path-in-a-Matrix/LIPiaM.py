# -*- coding=utf-8 -*-


def findEleP(matrix, row, col):
    try:
        if matrix[row][col]:
            return True, matrix[row][col]
    except:
        return False


def recordNum(matrix, *coodNum):
    row = coodNum[0]
    col = coodNum[1]
    reNum = 0
    largerIndex = []

    for i in [-1, 1]:
        if row + i < 0:
            pass
        else:
            rr = findEleP(matrix, row + i, col)
            if rr and rr[1] > matrix[row][col]:
                reNum += 1
                largerIndex.append([row + i, col])
        if col + i < 0:
            pass
        else:
            rr2 = findEleP(matrix, row, col + i)
            if rr2 and rr2[1] > matrix[row][col]:
                reNum += 1
                largerIndex.append([row, col + i])

    return reNum, largerIndex


def makeDic(matrix):
    row, col = 0, 0
    indexDic = {}
    for i in matrix:
        col = 0
        for ii in i:
            indexDic[(row, col)] = recordNum(matrix, row, col)
            col += 1
        row += 1
#        print(row, col)
    print(indexDic)

if __name__ == "__main__":
    makeDic([[9, 9, 4], [6, 6, 8], [2, 1, 1]])
