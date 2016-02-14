from math import factorial
import sys


def genList(n):
    resultList = []
    for i in range(n + 1):
        ii = i
        if (n - ii) % 2 == 0:
            resultList.append((ii, (n - ii) / 2))
    print(resultList)
    return resultList


def cNquM(n, m):
    nn = factorial(n)
    nn = nn / factorial(n - m)
    mm = factorial(m)
    return nn / mm


def CS(n):
    resultList = genList(n)
    allwaysNum = 0
    for ii in [i for i in resultList]:
        allwaysNum += cNquM(sum(ii), ii[0])
    return allwaysNum


def main(n):
    print("There are {0} ways".format(CS(n)))

if __name__ == "__main__":
    main(int(sys.argv[1]))
