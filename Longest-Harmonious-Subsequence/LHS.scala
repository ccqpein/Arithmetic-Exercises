def findLHS(nums:Array[Int]) = {
  var dic = Map[Int, Int]()
  for (num <- nums) {
    if (dic.contains(num))
      dic += (num -> (dic(num) + 1))
    else
      dic += (num -> 1)
  }

  val keys = dic.toList.sortWith{(x,y) => x._1 < y._1}
  var result = 0
  for {ind <- 1 to keys.length - 1
    a = keys(ind)._1
    b = keys(ind-1)._1
    if (a - b == 1);
    if (dic(a) + dic(b) > result)
    } result = dic(a) + dic(b)

  result
}

println(findLHS(Array(1,3,2,2,5,2,3,7)))
