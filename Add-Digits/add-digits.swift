func AddDigit(numIn:Int) -> Int {
    //var reNum:Int?
    let temp:String = String(numIn)
    //var charList = [Int]()
    var sumNum:Int = 0
    for c in temp.characters {
        let cc:Int? = Int(String(c))
        sumNum += cc!
    }

    print(sumNum)
    if sumNum < 10{
        return sumNum
    } else{
        return AddDigit(sumNum)
    }
}

AddDigit(19745)

