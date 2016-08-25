import Foundation

// Random extension function from http://stackoverflow.com/questions/37843647/shuffle-array-swift-3
extension Array {
    mutating func shuffle () {
        for i in (0..<self.count).reversed() {
            let ix1 = i
            let ix2 = Int(arc4random_uniform(UInt32(i+1)))
            (self[ix1], self[ix2]) = (self[ix2], self[ix1])
        }
    }
}


class Solution {
    var arr:[Int:Int] = [:]
    var ar:[Int]
    
    init(_ nums:[Int]) {
        self.ar = nums
        for (i,e) in nums.enumerated(){
            self.arr[i] = e
        }
    }

    func Reset() {
        for i in 0..<arr.count {
            self.ar[i] = self.arr[i]!
        }
    }

    func Shuffle() {
        self.ar.shuffle()
    }
    
}

var obj = Solution([1,2,3])
obj.Shuffle()
print(obj.ar)
obj.Reset()
print(obj.ar)
obj.Shuffle()
print(obj.ar)
