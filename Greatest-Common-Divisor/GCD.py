def remove_zero(l):
    return [i for i in l if i != 0]


def division(a, b):
    for i in range(2, min(a, b) + 1):
        if a % i == 0 and b % i == 0:
            return False
    return True


def GCD(l, ind):
    # if first input is non-zero list, this line can be delete
    inner_l = remove_zero(l)
    smallest = min(inner_l)
    if len(inner_l) == 1:
        print(smallest)
        return smallest, ind
    else:
        next_round = remove_zero(
            [i - smallest for i in inner_l])
        if len(next_round) == 1 and division(next_round[0], smallest):
            return 1, ind

        next_round.append(smallest)
        if next_round == inner_l:
            return smallest
        else:
            return GCD(next_round, ind + 1)


def GCD2(l, i):
    inner_l = remove_zero(l)
    smallest = 0
    while len(inner_l) != 1:
        i += 1
        smallest = min(inner_l)
        next_round = remove_zero(
            [i - smallest for i in inner_l])
        next_round.append(smallest)
        inner_l = next_round

    return smallest, i


# Euclid's Algorithm below

def GCD3(a, b):
    if b == 0:
        return a
    else:
        return GCD3(b, a % b)


def nGCD3(l, leng):
    if leng == 1:
        return l[0]

    return GCD3(l[leng - 1], nGCD3(l, leng - 1))
