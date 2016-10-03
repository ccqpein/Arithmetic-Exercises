let testStr = "25525511135"
let testStr2 = "0000"
let testStr3 = "010010"
let testStr4 = "101023"

class Solution {
    func cutIp(_ s:String) -> [[String]] {
        let lenStr:Int = s.characters.count
        let iterTime:Int = min(lenStr, 4)
        var reStrList = [[String]]()

        for i in 1...(iterTime - 1) {
            let startIndex = s.startIndex
            let endIndex = s.endIndex
            let cutIndex = s.index(s.startIndex, offsetBy:i)

            reStrList.append(
              [s[startIndex..<cutIndex],s[cutIndex..<endIndex]]
            )
        }
        //print(reStrList)
        return reStrList
    }

    func nextCut(_ strL:[String]) -> [[String]] {
        if strL.count == 1{
            return cutIp(strL.last!)
        }

        var reStrList = [[String]]()
        if strL.last!.characters.count == 1{
            reStrList.append(strL)
        }else {
            for i in cutIp(strL.last!) {
                var temp = strL.dropLast()
                temp.append(contentsOf:i)
                reStrList.append(
                  Array(temp)
                )
            }
        }

        return reStrList
    }
    
    func restoreIpAddresses(_ s:String) -> [String] {
        return ["a"]
    }
}


var a = Solution()
var testS = ["0", "000"]
print(a.nextCut(testS))
