var matrix1:[[Int]] = [
    [1, 2, 0, 3, 4, 5],
    [0, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6],
]

var matrix2:[[Int]] = [
    [1, 2, 3, 3, 4, 5],
    [0, 4, 5, 6, 7, 8],
    [1, 2, 3, 4, 5, 6],
]

func setZeroes(matrix m: inout [[Int]]) -> [[Int]] {
    var allRowZero:Set<Int> = []
    
    for r in 0..<m.count {
        var thisRowZero:Bool = false
        for (ri, c) in m[r].enumerated() {
            if c == 0 {
                thisRowZero = true
                allRowZero.insert(ri)
            }
        }
        if thisRowZero {
            m[r] = [Int](repeating:0, count:m[r].count)
        }
    }

    for r in 0..<m.count {
        for c in allRowZero {
            m[r][c] = 0
        }
    }

    return m

}

print(setZeroes(matrix:&matrix1))
print(setZeroes(matrix:&matrix2))
