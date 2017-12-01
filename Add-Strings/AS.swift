var dict1 = {() -> [Character:Int] in
    var dictInner = Dictionary<Character,Int>()
    var seed = 0
    for c in "0123456789" {
        dictInner[c] = seed
        seed += 1
    }
    return dictInner
}()
