func geneList(_ numInput:Int) -> [Int] {
    var reL = [Int]()
    for i in 1..<numInput {
        reL.append(i*i)
    }
    return reL
}

func ps(_ numInput:Int) -> Int{
    guard numInput != 0 else {return 0}

    var squareL = geneList(numInput)

    func innerPs(_ n:Int, _ sL:[Int]) -> Int{
        guard n != 0 else {return 0}
        var result = 1
        for i in sL.reversed() {
            if i <= n {
                print(i)
                result += innerPs((n-i),sL)
                break
            }
        }
        return result
    }
    var result = innerPs(numInput,squareL)
    return result
}

//print(ps(12)) //result should be 3, but I wrote wrong
