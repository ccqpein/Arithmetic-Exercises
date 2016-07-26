func MoveZeros(_ arr:[Int]) -> [Int] {
    var reArr:[Int] = []
    var zeroT:[Int] = []
    for i in arr {
        if i != 0 {
            reArr.append(i)
        }else {
            zeroT.append(0)
        }
    }
    reArr.append(contentsOf:zeroT)

    return reArr
}
