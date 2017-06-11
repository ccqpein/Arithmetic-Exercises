func nextGreaterElements(_ nums:[Int]) -> [Int]{
    var stack = [Int]()
    let len = nums.count
    var res = Array(repeating:-1,count:len)

    for _ in 0..<2 {
        for ind in 0..<len{
            while (!stack.isEmpty && nums[stack.last!] < nums[ind]){
                res[stack.last!] = nums[ind]
                stack.popLast()
            }
            stack.append(ind)
        }
    }
    return res
}

nextGreaterElements([1,2,1])
nextGreaterElements([5,3,4,6,7])
