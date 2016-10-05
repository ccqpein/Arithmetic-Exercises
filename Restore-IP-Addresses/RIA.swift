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
                temp.append(contentsOf:i) // temp is a slice
                reStrList.append(
                  Array(temp)
                )
            }
        }

        return reStrList
    }

    func mergeIp(_ l:[String]) -> String{
        var str = l.reduce("", {$0 + "." + $1})
        str.remove(at:str.startIndex)
        return str
    }

    func headZero (_ sL:[String]) -> Bool{
        for s in sL {
            if (s.characters[s.startIndex] == "0"
                  && s.characters.count > 1) {
                return false
            }
        }
        return true
    }
    
    func less255(_ sL:[String]) -> Bool{
        for s in sL {
            if Int(s)! > 255{
                return false
            }
        }
        return true
    }
    
    
    func restoreIpAddresses(_ s:String) -> [String] {
        var tempStr = [[s]]

        for _ in 1...3 {
            var temp = [[String]]()
            
            for l in tempStr {
                temp.append(contentsOf: self.nextCut(l))
            }
            tempStr = temp
        }

        var reStr:[String] = []
        for l in tempStr {
            if (l.count == 4 &&
                  self.less255(l) &&
                  self.headZero(l)) {
                reStr.append(self.mergeIp(l))
            }
        }
        
        //print(tempStr)
        return reStr
    }
}


var a = Solution()

print(a.restoreIpAddresses(testStr))
print(a.restoreIpAddresses(testStr2))
print(a.restoreIpAddresses(testStr3))
print(a.restoreIpAddresses(testStr4))
