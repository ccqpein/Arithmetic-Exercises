let dicOfLetters:[Character:Int] = [
    "I": 1,
    "V": 5,
    "X": 10,
    "D": 500,
    "L": 50,
    "C": 100,
    "M": 1000,
]

func romanToInt(Roman s:String) -> Int{
    let charList = s.characters
    var sumNumber = 0
    var char1:Int = 0 // var char1,char2:Int = 0
    var char2:Int = 0

    for i in charList {
        print(i)
        char2 = dicOfLetters[i]!
        sumNumber = sumNumber + char2

        switch (char2, char1) {
        case let(x,y) where (x == 5 || x == 10) && y == 1:
            sumNumber = sumNumber - 2
        case let(x,y) where (x == 1000 || y == 500) && y == 100:
            sumNumber = sumNumber - 200
        case let(x,y) where (x == 100 || y == 50) && y == 10:
            sumNumber = sumNumber - 20
        default:
            break
        }
        char1 = char2
    }
    return sumNumber
}


romanToInt(Roman:"LXXX")
