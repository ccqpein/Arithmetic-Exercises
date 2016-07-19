//Using swift3 beta

import Foundation

var testS:String = "bbc abcdab abcdabcdabde"
var testP:String = "abcdabd"

func makeIndexTable(pattern: String) -> Array<Int>{
    
    let patternA = Array(pattern.characters)
    var indexTable:Array<Int> = [-1,]
    for i in 0..<patternA.count {
        let thisP = patternA[0...i]
        //print(thisP)
        var tempLen = 0
        for ii in 0..<i {
            if (thisP[0...ii] == thisP[i-ii...i])
               && (thisP[0...ii].count > tempLen) {
                tempLen = thisP[0...ii].count
            }
        }
        indexTable.append(tempLen)
    }
    indexTable.removeLast()
    return indexTable
}

func doKmp (str:String, pattern:String) ->Bool{
    let pIndexT = makeIndexTable(pattern:pattern)
    let pA = Array(pattern.characters)
    let lenIndexT = pIndexT.count
    var pI = 0
    
    let strA = Array(str.characters)
    let lenStr = strA.count
    var sI = 0

    while pI != lenIndexT && sI != lenStr {
        if (strA[sI] == pA[pI]){
            pI += 1
            sI += 1
        }else if (pI == 0){
            sI += 1
        }else{
            pI = pIndexT[pI]
        }
    }

    var result = false
    return {() -> Bool in
               if pI == lenIndexT {
                   return true
               }else{
                   return false
               }
           }
//    return result
}

print(makeIndexTable(pattern:testP))

print(doKmp(str:testS, pattern:testP))
