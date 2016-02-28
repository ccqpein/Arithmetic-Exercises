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

    for i in [-1, 1]:
        rr = findEleP(matrix, row + i, col)
        if rr and rr[1] > matrix[row][col]:
            reNum += 1
        rr2 = findEleP(matrix, row, col + i)
        if rr2 and rr2[1] > matrix[row][col]:
            reNum += 1
