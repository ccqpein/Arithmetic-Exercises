func ham(_ num:Int) -> Int {
    var oneSet = 1
    var n = num
    while(n>1) {
        oneSet += n%2
        n = n/2
        print(n)
    }
    return oneSet
}
