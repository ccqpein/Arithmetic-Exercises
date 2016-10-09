let str = "hello world"

class Solution {
    func deleteEndingSpace(_ s:String) -> String{
        
    }

    func lengthOfLastWord(_ s: String) -> Int {
       var reStr = ""
    
        for i in s.characters.reversed() {
            if i != " "{
                reStr.insert(i, at:s.startIndex) 
            }else{  
                break
            }
        }
    return reStr.characters.count
    }
}
