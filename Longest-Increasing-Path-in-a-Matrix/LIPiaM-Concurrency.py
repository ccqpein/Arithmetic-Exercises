# -*- coding=utf-8 -*-
import sys
# import threading
import multiprocessing as mul


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
#    print(indexDic)
    return indexDic


def findLongest(matrix, indexDic, row, col):
    if indexDic[(row, col)][0] == 0:
        return 1
    elif indexDic[(row, col)][0] == 1:
        value = 1 + findLongest(matrix, indexDic,
                                indexDic[(row, col)][1][0][0],
                                indexDic[(row, col)][1][0][1])
        return value
    elif indexDic[(row, col)][0] == 2:
        value1 = 1 + findLongest(matrix, indexDic,
                                 indexDic[(row, col)][1][0][0],
                                 indexDic[(row, col)][1][0][1])
        value2 = 1 + findLongest(matrix, indexDic,
                                 indexDic[(row, col)][1][1][0],
                                 indexDic[(row, col)][1][1][1])
        if value1 >= value2:
            return value1
        else:
            return value2

    elif indexDic[(row, col)][0] == 3:
        value1 = 1 + findLongest(matrix, indexDic,
                                 indexDic[(row, col)][1][0][0],
                                 indexDic[(row, col)][1][0][1])
        value2 = 1 + findLongest(matrix, indexDic,
                                 indexDic[(row, col)][1][1][0],
                                 indexDic[(row, col)][1][1][1])
        value3 = 1 + findLongest(matrix, indexDic,
                                 indexDic[(row, col)][1][2][0],
                                 indexDic[(row, col)][1][2][1])
        return max(value1, value2, value3)


if __name__ == "__main__":
    #    matrix = [[9, 9, 4], [6, 6, 8], [2, 1, 1]]
    if len(sys.argv) == 1:
        #matrix = [[3, 4, 5], [3, 2, 6], [2, 2, 1]]
        matrix = [[3, 4, 5, 3, 2, 1], [2, 3, 4, 5, 2, 6], [1, 2, 4, 2, 2, 1],
                  [2, 3, 4, 1, 5, 6], [2, 4, 1, 3, 5, 1], [2, 4, 1, 3, 5, 2]]
    else:
        matrix = sys.argv[1]  # It not work

    indexDic = makeDic(matrix)
    resutleTemp = []
    pool = mul.Pool(4)
    for i in range(len(matrix)):
        for ii in range(len(matrix[i])):
            # print(findLongest(matrix, indexDic, i, ii))
            resutleTemp.append(
                pool.apply_async(findLongest, args=(matrix, indexDic, i, ii)))
            # resutleTemp.append(a)
    pool.close()
    pool.join()
    resulte = []
    for r in resutleTemp:
        resulte.append(r.get())
    print(max(resulte))
