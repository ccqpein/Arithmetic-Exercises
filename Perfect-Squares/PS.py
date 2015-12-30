def numSquares(n):
    squaresSumList = [i * i for i in xrange(n)]
    squaresSumList2 = []
    rTime = 0
    for summ in squaresSumList:
        if summ <= n:
            squaresSumList2.append(sum)
            continue
    squaresSumList = squaresSumList2
    squaresSumList2.reverse()
    for summ2 in squaresSumList2:
        rTime = 2
        for summ in squaresSumList:
            xx = summ + summ2
            if xx == n:
                return rTime
