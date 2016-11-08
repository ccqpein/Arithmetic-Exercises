func countSmaller (nums:[Int]) -> [Int] {
    let lenArr:Int = nums.count
    var reArr:[Int] = []
    for i in 0..<lenArr {
        let thisNum = nums[i]
        let startIndexNum = i + 1
        var nn = 0
        for ii in startIndexNum..<lenArr {
            if nums[ii] < thisNum {
                nn += 1
            }
        }
        reArr.append(nn)
    }
    print(reArr)
    return reArr
    
}

countSmaller([5,2,6,1])
