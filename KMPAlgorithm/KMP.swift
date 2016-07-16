//Using swift3 beta

import Foundation

var testS:String = "bbc abcdab abcdabcdabde"
var testP:String = "abcdabd"

func makeIndexTable(pattern: String) -> Array<Int>{

    let patternA = Array(pattern.characters)
    var indexTable:Array<Int> = [-1,]
    for i in 1..<patternA.count {
                     let thisP = patternA[0...i]
                     //print(thisP)
                     var tempLen = 0
                     for ii in 0..<i {
                                       //print(thisP[0...ii], thisP[i-ii...i])
                                       if (thisP[0...ii] == thisP[i-ii...i])
                                          && (thisP[0...ii].count > tempLen) {
                                           tempLen = thisP[0...ii].count
                                       }
                               }
                                   indexTable.append(tempLen)
             }
                 return indexTable
                                  
}

print(makeIndexTable(pattern:testP))

//print(Array(testP.characters))
//print(testP.endIndex)
//let myRange = testP.startIndex..<testP.index(testP.startIndex, offsetBy:4)
//print(testP.substring(with:myRange))
