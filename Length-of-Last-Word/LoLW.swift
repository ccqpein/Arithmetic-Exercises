let str = "hello world"

func lolw(_ str:String) -> Int {
    var reStr = ""

    for i in str.characters.reversed() {
        if i != " "{
            reStr.insert(i, at:str.startIndex) 
        }else{
            break
        }
    }
    return reStr.characters.count
}
