func quickSort<T:Comparable>(_ a:[T]) -> [T] {
    guard a.count > 1 else {return a}

    let key = a.first!
    let headL = quickSort(a[1..<a.endIndex].filter{$0 < key})
    let tailL = quickSort(a[1..<a.endIndex].filter{$0 >= key})
    
    let result = headL + [key] + tailL
    
    return result
}

var a = [10,2,3,4,5]
print(quickSort(a))
