def num_trees(num):
    a = {0: 1, 1: 1}
    for i in range(2, num+1):
        for j in range(0, i):
            try:
                a[i] += a[j] * a[i-j-1]
            except:
                a[i] = a[j] * a[i-j-1]
    return a[num]
