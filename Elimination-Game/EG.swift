func lastRemaining(_ n:Int) -> Int {
    var l:[Int] = []

    for i in 1...n {
        l.append(i)
    }

    while l.count != 1 {
        var tempL:[Int] = []
        var tempL2:[Int] = []
        
        for (i,e) in l.enumerated() {
            if i%2 != 0{
                tempL.append(e)
            }
        }

        if tempL.count == 1{
            l = tempL
            break
        }

        if tempL2.count%2 == 0{
            for (i,e) in tempL.enumerated() {
                if i%2 == 0 {
                    tempL2.append(e)
                }
            }
        }else {
            for (i,e) in tempL.enumerated() {
                if i%2 != 0 {
                    tempL2.append(e)
                }
            }
        }

        l = tempL2
    }

    return l[0]
}


lastRemaining(9)
