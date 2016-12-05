func soe (_ n:Int) -> [Int] {
    var nL = {(n:Int) -> [Int] in
        var LTemp = [Int]()
        for i in 1...n{
            LTemp.append(i)
        }
        return LTemp
    }(n)

    var endNum = Double(n).squareRoot()
    endNum.round()

    for i in 2...Int(endNum) {
        nL = nL.filter {$0 == i || $0 % i != 0}
    }

    nL.removeFirst()

    return nL
}
