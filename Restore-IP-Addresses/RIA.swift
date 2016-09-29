let testStr = "25525511135"
let testStr2 = "0000"
let testStr3 = "010010"
let testStr4 = "101023"

class Solution {
    func cutIp(_ s:String) -> [[String]] {
        let lenStr:Int = s.characters.count
        var iterTime:Int = min(lenStr, 4)
        var reStrList = [[String]]()

        for i in 1...(iterTime - 1) {
            var startIndex = s.startIndex
            var endIndex = s.endIndex
            var cutIndex = s.index(s.startIndex, offsetBy:i)

            reStrList.append(
              [s[startIndex..<cutIndex],s[cutIndex..<endIndex]]
            )
        }
        print(reStrList)
        return reStrList
    }
    
    func restoreIpAddresses(_ s: String) -> [String] {
        return ["a"]
    }
}

