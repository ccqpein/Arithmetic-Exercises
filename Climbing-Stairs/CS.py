from math import factorial


def genList(n):
    resultList = []
    for i in xrange(n):
        ii = i + 1
        if (n - ii) % 2 == 0:
            resultList.append((ii, (n - ii) / 2))
    print(resultList)
    return resultList


def cNquM(n, m):
    nn = factorial(n)
    nn = nn / factorial(n - m)
    mm = factorial(m)
    return nn / mm


def main():
    print("ddd")
