val test1 = Array(
  Array(2),
  Array(3,4),
  Array(6,5,7),
  Array(4,1,8,3),
)

def miniTotal(b:Array[Array[Int]], a:Int=0):Int = {
  if (b.isEmpty)
    a
  else {
    miniTotal(b.tail, b.head.map(x => x+a).min)
  }
}

println(miniTotal(test1))
println(miniTotal(Array(Array(2))))
