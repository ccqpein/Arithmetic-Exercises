let example1:[[Int?]] = [
    [0, 6, 3, nil, nil, nil,],
    [6, 0, 2, 5, nil, nil,],
    [3, 2, 0, 3, 4, nil,],
    [nil, 5, 3, 0, 2, 3,],
    [nil, nil, 4, 2, 0, 5],
    [nil, nil, nil, 3, 5, 0,],
]

let example2:[[Int?]] = [
    [0, 7, 9, nil, nil, 14,],
    [7, 0, 10, 15, nil, nil,],
    [9, 10, 0, 11, nil, 2],
    [nil, 15, 11, 0, 6, nil,],
    [nil, nil, nil, 6, 0, 9,],
    [14, nil, 2, nil, 9, 0,],
]

struct PathResult {
    var val:Int
    var pathList:Array<Int>
}

enum GetValError:ErrorProtocol{
case noVal(String)
}

func delete(_ ele: inout Int, _ array: inout [Int]){
    var removeL = [Int]()
    for i in (0..<array.count) {
        if array[i] == ele{
            removeL.append(i)
        }
    }
    removeL = removeL.reversed()
    for i in removeL {
        array.remove(at:i)
    }
}

func getVal(pathRoot:[Int], pathP:Int, m:[[Int?]] = example1) throws -> PathResult{
    var pathTemp = pathRoot
    pathTemp.append(pathP)
    var pathVal = 0
    let valTest = m[pathRoot.last!][pathP]
    guard valTest != nil else{
        throw GetValError.noVal("No value")
    }
    for i in (1..<pathTemp.count) {
        pathVal = pathVal + m[pathTemp[i-1]][pathTemp[i]]!
    }
    return PathResult(val:pathVal, pathList:pathTemp)
}

func dijkstra(start:Int, matrix m:[[Int?]]) -> [PathResult]{
    // initialize
    var s:[Int] = [start]
    var u:[Int] = []
    for i in 0..<m.count where i != start {
        u.append(i)
    }

    // make a temp list for result
    let startResult = PathResult(val:0, pathList:[start])
    var smallL:[PathResult] = [startResult]
    var largeL:[PathResult] = []

    for t in 0..<u.count {
        var tempResultList:[PathResult] = []
        for e in u {
            do{
                thisPath = try getVal(pathRoot:smallL.last!.pathList, pathP:e)
                var tempOldVal:[Int] = []
                for oldP in largeL where oldP.pathList.last! == e {
                    tempOldVal.append(oldP.val)
                }
                if thisPath.val < tempOldVal.min() {
                    tempResultList.append(thisPath)
                }else{
                    
                }
            }catch GetValError.noVal(){
                continue
            }
        }
        tempResultList.sort{ $0.val < $1.val }
        smallL.append(tempResultList.first!)
        largeL = largeL + tempResultList.dropFirst()
    }
}


/*var test1 = [0,1,2]
var test2 = 5*/

var test1 = [0,1,]
var test2 = 2

do{
    print(try getVal(pathRoot:test1, pathP:test2))
}catch GetValError.noVal(let errs) {
    print(errs)
}

//print(example1)
//print(example2)
