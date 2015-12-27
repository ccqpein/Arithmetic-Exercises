def isUgly(num):
    while(1):
        if num == 1 or num == 2 or num == 3 or num == 5:
            return True
        elif num == 0:
            return False
        elif num % 2 == 0:
            num = num / 2
            continue
        elif num % 3 == 0:
            num = num / 3
            continue
        elif num % 5 == 0:
            num = num / 5
            continue
        else:
            return False
