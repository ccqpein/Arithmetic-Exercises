class Solution {
    func totalHammingDistance(_ nums: [Int]) -> Int {
        var result = 0
        for i in 0..<nums.count {
            for ii in (i+1)..<nums.count{
                result += diffHam(nums[i],nums[ii])
            }
        }
        return result
    }

    func diffHam(_ n1:Int, _ n2:Int) -> Int {
        var a = n1
        var b = n2
        var result = 0
        while(a != 0 || b != 0){
            if a%2 != b%2{
                result += 1
            }
            (a,b) = (a/2,b/2)
        }
        return result
    }

}
