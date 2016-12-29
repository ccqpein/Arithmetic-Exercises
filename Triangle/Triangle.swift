class Solution {
    func minimumTotal(_ triangle: [[Int]]) -> Int {
        var lastList = triangle[0]
        var tempList = [Int]()
        for row in triangle[1..<triangle.endIndex] {
            var lastIndex = lastList.count
            let rowCount = row.count
            for i in 0..<lastIndex{
                var tempRow = row.map {$0 + lastList[i]}
                tempList.append(contentsOf: tempRow)
            }
            (lastList, tempList) = (tempList, [Int]())
        }
        lastList.sort(by:<)
        return lastList[0]
    }
}

var a = Solution()
a.minimumTotal([
     [2],
    [3,4],
   [6,5,7],
  [4,1,8,3]
])
