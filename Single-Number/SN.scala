def singerNum(nl:Array[Int]):Int = {
  var hst:Map[Int,Int] = Map()

  for (num <- nl) {
    val thisVal = if (hst.contains(num)) hst.get(num).get else 0
    hst += (num -> (thisVal + 1))
  }

  println(hst.mkString(", "))

  var result = 0
  for ((k, v) <- hst){
    if (v == 1)
      result = k
  }
  result
}

print(singerNum(Array(1,1,2,3,3,4,4,5,5,6,6,7,7)))
