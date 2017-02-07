def GA2(data):
    sortData = data
    sortData.sort(key=lambda x: x[1])
    result = [[sortData[0]]]
    for i in sortData[1:]:
        temp = []
        for j in range(len(result)):
            if result[j][-1][1] <= i[0]:
                temp.append(j)
                break
        try:
            result[temp[0]].append(i)
        except:
            result.append([i])
    return result

if __name__ == "__main__":
    testData = [(1, 2), (2, 4), (3, 4), (3, 6), (3, 5),
                (4, 7), (4, 5), (7, 8), (6, 9)]
    print(GA2(testData))
