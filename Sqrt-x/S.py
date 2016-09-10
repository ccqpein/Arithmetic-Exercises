def mySqrt(x):
    if x == 0:
        return 0
    elif x == 1:
        return 1

    # use 1
    result = 0
    result2 = 1  # = 1.0 if using in python2
    while result2 != result:
        result = result2
        result2 = (result + x / result) / 2

    return int(result2)
