func AddDigit(numIn:Int) -> Int {
    //var reNum:Int?
    let temp:String = String(numIn)
    var charList = [String]()
    for c in temp.characters {
        let cc = String(c)
        charList += [cc]
        print(charList)
    }

    return 10
}

func main(){

    AddDigit(15)
}
    /*
    for c in charList {
        reNum = Int(c) + reNum
    }

    if reNum<10 {
        return reNum
    }else{
        reNum = AddDigit(reNum)
        return reNum
    }*/

