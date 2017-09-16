def mergeInner(l1, l2):
    if l1 == []:
        return l2

    if l1[0] <= l2[0]:
        return [l1[0]] + mergeInner(l1[1:], l2)
    else:
        return [l2[0]] + mergeInner(l2[1:], l1)


def mergeSort(l1):
    a = [l1[i] for i in range(len(l1) // 2)]
    b = [l1[i] for i in range(len(l1) // 2, len(l1))]

    if len(l1) >= 3:
        return mergeInner(mergeSort(a), mergeSort(b))
    else:
        return mergeInner(a, b)


print(mergeSort([3, 41, 52, 26, 38, 57, 9, 49]))
