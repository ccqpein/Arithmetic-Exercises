func IsHappy (n:Int)-> Int {
    var sumNum:Int = 0
    let nn = String(n)
    for c in nn.characters {
        let cc = Int(String(c))
        sumNum = sumNum + cc! * cc!
    }

    //print(sumNum)
    if sumNum == 1{
        print("It is happy number")
        return sumNum
    } else if sumNum >= 10 {
        return IsHappy(sumNum)
    }else {
        print("It is not happy number")
        return sumNum
    }
}

IsHappy(19)
