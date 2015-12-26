def isHappy(n):
    numList = []
    num = n
    numListRemember = {0}

    while(1):
        numList = []
        for i in str(num):
            numList.append(int(i))
        num = sum(list(map(lambda x: x ** 2, numList)))
        if num in numListRemember:
            return False
        elif num == 1:
            return True
        else:
            numListRemember.add(num)
