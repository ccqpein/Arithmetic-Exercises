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

func do-kmp (str:String, pattern:String) ->Bool{
    let pIndexT = makeIndexTable(pattern)
    let pA = Array(pattern)
    let lenIndexT = pIndexT.count
    var pI = 0
    
    let strA = Array(str)
    let lenStr = strA.count
    var sI = 0
    
    while (pI!= lenIndexT) {

        if (strA[sI]== pA[pI]){
            pI++
            sI++
        }else {
            pI = pI- 
        }
    }
}

print(makeIndexTable(pattern:testP))

//print(Array(testP.characters))
//print(testP.endIndex)
//let myRange = testP.startIndex..<testP.index(testP.startIndex, offsetBy:4)
//print(testP.substring(with:myRange))
