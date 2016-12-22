var test1 = [Int]()  // return 0
var test2 = [2, 1, 2, 1, 0, 1, 2]  // return 2
var test3 = [1]  // return 0
var test4 = [7, 1, 5, 3, 6, 4]  // return 5
var test5 = [2, 4, 1]  // return 2

func maxProfit(_ prices:[Int]) -> Int {
    var maxT = 0
    var result = 0
    guard prices.count > 1 else {return 0}
    var minT = prices[0]

    for i in prices {
        switch (i - minT) {
        case let a where a < 0:
            minT = i
        case let a where a > result:
            maxT = i
            result = maxT - minT
        default:
            continue
        } 
    }

    result = (result<0) ? 0 : result
    return result
}
