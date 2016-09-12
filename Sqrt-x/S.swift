func mySqrt(_ x:Int) -> Int{
    var xTemp:Double?
    
    switch x {
    case 0,1 : return x
    default: xTemp = Double(x)
    }

    var re1:Double = 0
    var re2:Double = 1

    while re2 != re1 {
        re1 = re2
        re2 = (re1 + xTemp! / re1) / 2
    }

    return Int(re2)
}


mySqrt(0)
mySqrt(1)
mySqrt(125348)
