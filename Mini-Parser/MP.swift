class NestedInteger {
    var value:Int?
    var nestedL:NestedInteger?
}

func deserialize(_ s:String) -> NestedInteger {
    let a = NestedInteger()
    var str = Array(s.characters)

    if str[0] != "[" {
        let val = Int(s)!
        a.value = val
        return a
    }else{
        var ss = str[1..<str.endIndex-1]
        var breakInd = 1
        var inTemp:[Character] = []

        lo:for v in ss {
            switch v {
            case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9":
                inTemp.append(v)
            case "[":
                breakInd = ss.index(of:v)!
                break lo
            default:
                continue
            }
        }

        let val = Int(String(inTemp))
        let nextss = String(ss[breakInd..<ss.endIndex])
        //print(nextss)

        a.value = val
        a.nestedL = deserialize(nextss)

        return a
    }
}

var s1 = "324"
var s2 = "[123,[456,[789]]]"


/**
print(deserialize(s1).value)
print(deserialize(s1).nestedL)
print(deserialize(s2).value)
print(deserialize(s2).nestedL!.value)
*/
