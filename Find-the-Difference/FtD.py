l = [i + 1 for i in range(9)]

while True:
    lleng = len(l)
    tempL = []
    for i in range(lleng):
        if i % 2 == 0:
            tempL.append(l[i])

    l = tempL.reverse()
    print(l)
    if len(l.sort()) == 1:
        break
