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
    testData2 = [(9, 10.5), (9, 12.5), (9, 10.5), (11, 12.5), (11, 14),
                 (13, 14.5), (13, 14.5), (14, 16.5), (15, 16.5), (15, 16.5)]
    print(GA2(testData2))
