coinList = [1, 5, 10, 25]
resultList = {}


def changCoin(value, coinL, resultL):
    if value in coinL:
        resultL[value] = 1
        return 1

    try:
        resultL[value] > 1
        return resultL[value]
    except:
        pass

    tempResult = []
    for i in [c for c in coinL if c <= value]:
        numCoins = 1 + changCoin(value - i, coinL, resultL)
        print(numCoins)
        tempResult.append(numCoins)

    resultL[value] = min(tempResult)
    return min(tempResult)
