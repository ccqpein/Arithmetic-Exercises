class Solution {
    func detectCapitalUse(_ word: String) -> Bool {
        var flag:Bool?
        guard word.characters.count > 1 else {return true}
        
        for sInd in word.characters.indices {
            let thisChar = String(word[sInd])
            //print(thisChar)

            if sInd == word.startIndex{
                if thisChar.uppercased() != thisChar{
                    flag = false
                }
            }else if sInd == word.index(word.startIndex,offsetBy:1) && flag == nil{
                if thisChar.uppercased() != thisChar{ // low case
                    flag = false
                }else{
                    flag = true
                }
            }else if thisChar.uppercased() != thisChar && flag == true{
                return false
            }else if thisChar.uppercased() == thisChar && flag == false{
                return false
            }
        }
        
        return true
    }
}

let a = Solution()
a.detectCapitalUse("USA") // true
a.detectCapitalUse("FlaG") // false
a.detectCapitalUse("ggg") // true
