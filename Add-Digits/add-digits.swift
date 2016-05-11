func AddDigit(numStr:Int) -> Int {
    var reNum:Int
    let temp:String = String(numStr)
    for char in temp.characters {
        reNum += Int(char)
    }

    if reNum<10 {
        return reNum
    }else{
        reNum = AddDigit(reNum)
    }
}
