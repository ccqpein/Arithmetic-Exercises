func IsUgly(num:Int) -> String {
    var reS:String?
    switch num {
    case 1,2,3,5 :
        reS = "True"
    case let n where (n % 2 == 0):
        let newNum =  n % 2
        reS = IsUgly(newNum)
    default:
        return "oh yeah?"
    }

    return reS!
}

IsUgly(4)
