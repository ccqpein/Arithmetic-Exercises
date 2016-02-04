from math import factorial


def cNquM(n, m):
    nn = factorial(n)
    nn = nn / factorial(n - m)
    mm = factorial(m)

    return nn / mm
