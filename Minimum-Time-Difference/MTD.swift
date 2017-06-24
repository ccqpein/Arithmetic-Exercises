import Foundation

func findMinDifference(_ timePoints:[String]) -> Int{
    var minDict = [Int:Set<Int>]()
    for hour in 0...24 {
        minDict[hour] = Set<Int>()
    }
    
    for time in timePoints {
        let thisTime = time.components(separatedBy:":")
        let (hour, mins) = (Int(thisTime[0])!, Int(thisTime[1])!)

        if (minDict[hour]?.contains(mins))! {
            return 0
        }else{
            minDict[hour]?.insert(mins)
        }
    }
    minDict[24] = minDict[0]
    //print(minDict)

    var small = 1339
    for hour in 0..<24 {
        var tempList = Array<Int>()
        if !minDict[hour]!.isEmpty {
            tempList = Array(minDict[hour]!).sorted()
            if !minDict[hour+1]!.isEmpty {
                tempList.append(minDict[hour+1]!.min()!+60)
            }
        }else {
            continue
        }
        
        for ind in 1..<tempList.count {
            if tempList[ind] - tempList[ind-1] < small{
                small = tempList[ind] - tempList[ind-1]
            }
        }
    }
    
    return small
}

findMinDifference(["23:59", "00:00", "22:34"])
findMinDifference(["22:27", "18:42", "09:57", "09:24", "09:26"])
findMinDifference(["23:59", "00:00", "22:34","22:34"])
