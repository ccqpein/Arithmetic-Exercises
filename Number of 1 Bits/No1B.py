def ham(n):
    oneSet = 0
    while(1):
        if n == 1:
            oneSet += 1
            break
        elif n % 2 == 1:
            oneSet += 1
            n = n // 2
        else:
            n = n / 2
    return oneSet
