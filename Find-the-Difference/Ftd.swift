let s1 = "abc"
let p1 = "abcd"

let s2 = "aaa"
let p2 = "abaa"

func FindTheDifferent(_ s:String, _ p:String) -> Character {
    var sT:[Character:Int] = [:]
    var pT:[Character:Int] = [:]

    for c in s.characters {
        if sT[c] == nil {
            sT[c] = 1
        }else{
            sT[c] = sT[c]! + 1
        }
    }

    for c in p.characters {
        if pT[c] == nil {
            pT[c] = 1
        }else{
            pT[c] = pT[c]! + 1
        }
    }

    for (c,v) in pT {
        if sT[c] == nil || sT[c] != v {
            return c
        }
    }

    return Character("")
}


print(FindTheDifferent(s1,p1))
print(FindTheDifferent(s2,p2))

