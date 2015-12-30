def numSquares(n):
    squaresSumList = [i * i for i in xrange(n + 1)]
    squaresSumList2 = []
    for summ in squaresSumList:
        if summ <= n:
            squaresSumList2.append(summ)
    squaresSumList = squaresSumList2
    for i in xrange(1, 10):
        if n in geneTable(i, squaresSumList):
            return True, i
    return "something wrong"


def geneTable(n, squaresSumList):
    squaresSumList2 = squaresSumList
    while(n - 1):
        squaresSumList3 = []
        for summ in squaresSumList:
            for summ2 in squaresSumList2:
                squaresSumList3.append(summ + summ2)
        squaresSumList2 = squaresSumList3
        n -= 1
#    print(squaresSumList2)
    return squaresSumList2
