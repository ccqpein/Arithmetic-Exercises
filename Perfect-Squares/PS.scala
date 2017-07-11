val test1 = 12
val test2 = 13

def numSquares(num:Int):Int = {
  val lst = for (i <- 1 to num if (i * i < num)) yield (i*i)
  var toCheck = Set[Int](num)
  var cnt = 0

  while (!toCheck.isEmpty) {
    var temp = Set[Int]()
    cnt += 1

    for (check <- toCheck) {
      if (toCheck.contains(0))
        return 0
      else if (lst.contains(check))
        return cnt
      else {
        for (p <- lst if (p < check)) {
          temp += check - p
        }
      }
    }
    toCheck = temp
  }

  return cnt
}

println(numSquares(12))
println(numSquares(13))
