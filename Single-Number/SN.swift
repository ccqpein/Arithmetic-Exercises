func SN(_ nL:[Int]) -> Int {
    var tb = [Int: Int]()

    for i in nL {
        if tb[i] != nil {
            tb[i]! += 1
        }else{
            tb[i] = 1
        }
    }

    for (k,v) in tb {
        if v != 2 {
            return k
        }
    }

    return 0
}

