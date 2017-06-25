def removeEle[T](vl:Array[T], v:T):Array[T] = vl.filter(_ != v)

println(removeEle(Array(4,5,4,6,7,8,2,3,4),4).mkString(" "))
