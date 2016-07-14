import Foundation

var testS:String = "bbc abcdab abcdabcdabde"
var testP:String = "abcdabd"

print(testS, testP)

/*func makeIndexTable(pattern:String) {

    for i in  {
        print(pattern[i])
             }
                                  
}*/

//print(Array(testP.characters))
print(testP.endIndex)
let myRange = testP.startIndex..<testP.index(testP.startIndex, offsetBy:4)
print(testP.substring(with:myRange))
