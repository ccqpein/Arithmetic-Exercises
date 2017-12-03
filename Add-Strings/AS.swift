var dict1 = {() -> [Character:Int] in
    var dictInner = Dictionary<Character,Int>()
    var seed = 0
    for c in "0123456789" {
        dictInner[c] = seed
        seed += 1
    }
    return dictInner
}()

var dict2 = {() -> [Int:Character] in
    var dictInner = Dictionary<Int,Character>()
    var seed = 0
    for c in "0123456789" {
        dictInner[seed] = c
        seed += 1
    }
    return dictInner
}()


func addString(numS1:String, numS2:String) -> String{
    var sum = 0
    for numS in [numS1,numS2]{
        for ind in 0..<numS.count {
            sum += (dict1[Array(numS)[ind]]! * {Int -> Int in
                        var base = 1
                        for _ in 0..<(numS.count-1-ind){
                            base *= 10
                        }
                        return base
                    }(ind))
        }
    }

    var reStr = ""
    while sum >= 10 {
        reStr = (String(dict2[sum%10]!) + reStr)
        sum = sum / 10
    }


    return (String(dict2[sum]!) + reStr)
}


addString(numS1:"0",numS2:"0")
addString(numS1:"99",numS2:"9")
