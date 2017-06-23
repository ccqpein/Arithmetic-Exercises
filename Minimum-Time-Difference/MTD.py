class Solution(object):

    def findMinDifference(self, timePoints):
        """
        :type timePoints: List[str]
        :rtype: int
        """

        minDict = {}  # make dict to store all time by Set
        for s in timePoints:
            cutStr = s.split(":")
            try:
                if int(cutStr[1]) in minDict[cutStr[0]]:
                    return 0  # if this time already in Set, return 0 directly
                else:
                    minDict[cutStr[0]].add(int(cutStr[1]))
            except:
                minDict[cutStr[0]] = {int(cutStr[1])}
        print(minDict)

        tempList = []  # change all time to pure minutes
        for hour in sorted(minDict):
            print(hour)
            count = int(hour)
            tempList += [60 * count + i for i in sorted(list(minDict[hour]))]
        # add first hour to end of list again, because it might be circle time
        firstHour = sorted(minDict)[0]
        tempList += [(24 + int(firstHour)) * 60 +
                     i for i in list(minDict[firstHour])]
        print(tempList)

        min = 1339  # a day have 1440 minutes
        for ind in range(1, len(tempList)):
            if abs(tempList[ind] - tempList[ind - 1]) < min:
                min = abs(tempList[ind] - tempList[ind - 1])
        return min


a = Solution()
print(a.findMinDifference(["23:59", "00:00", "22:34"]))  # 1
print(a.findMinDifference(["22:27", "18:42", "09:57", "09:24", "09:26"]))  # 2
