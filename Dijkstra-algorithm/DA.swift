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

struct pathResult {
    var val:Int?
    var pathList:Array<Int>?
}

func delete<T>(_ ele: inout T, _ array: inout [T]){
    for object in array{
        if object == ele{
            array.removeAtIndex(array.indexOf(ele)!)
        }
    }
}

func dijkstra(start:Int, matrix m:[[Int?]]) -> [pathResult]{
    var s:[Int] = [start]
    var u:[Int] = m.count
}

print(example1)
print(example2)
