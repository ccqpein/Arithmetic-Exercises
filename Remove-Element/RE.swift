func removeEle (numL:[Int], ele:Int) -> [Int] {
    let renumL = numL.filter(){$0 != ele}
    print(renumL)
    return renumL
}

removeEle([1,2,3], ele:2)
