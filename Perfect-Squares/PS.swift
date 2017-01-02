func geneList(_ numInput:Int) -> [Int] {
    var reL = [Int]()
    for i in 1..<numInput {
        reL.append(i*i)
    }
    return reL
}

func ps(_ numInput:Int) -> Int{
    guard numInput != 0 else {return 0}

    let squareL = geneList(numInput)
    var timeRe = 0
    for i in squareL.reverse() {
        if i <= numInput {
            timeRe += 1
            
        }
    }
}
