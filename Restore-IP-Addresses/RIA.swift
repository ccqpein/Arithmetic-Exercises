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

        if strL.last!.count == 1{
            return Array<[String]>(strL)
        }

        var reStrList = [[String]]()
        for i in cutIp(strL.last!) {
            reStrList.append(
              Array(strL.dropLast(),i))
        }
    }
    
    func restoreIpAddresses(_ s:String) -> [String] {
        return ["a"]
    }
}


var a = Solution()
a.nextCut(["0", "000"])
