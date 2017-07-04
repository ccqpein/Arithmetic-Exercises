def compare[T](func:(T,T)=>Boolean)(a:T,b:T) = func(a,b)

val larger = compare((a:Int,b:Int) => a > b)_
val smaller = compare((a:Int,b:Int) => a < b)_

def swapTwo[T](xs:Array[T],a:Int,b:Int) = {
  val temp = xs(a)
  xs(a) = xs(b)
  xs(b) = temp
  xs
}

def heapify(func:(Int,Int)=>Boolean, nl:Array[Int], ind:Int) = {
  val parent = nl(ind)
  val leftind = ind * 2 + 1
  val left = if (leftind >= nl.length) Some(None) else Some(nl(leftind))
  val rightind = ind * 2 + 2
  val right = if (rightind >= nl.length) Some(None) else Some(nl(rightind))

  if (left.get == None)
    (nl, null)
  else if (func(parent,left.getOrElse(0).asInstanceOf[Int])
    && (if (right.get == None) true
    else func(parent,right.getOrElse(0).asInstanceOf[Int])))
    (nl, null)
  else if (func(left.getOrElse(0).asInstanceOf[Int],parent)
    && (if (right.get == None) true
    else func(left.getOrElse(0).asInstanceOf[Int],right.getOrElse(0).asInstanceOf[Int])))
    (swapTwo(nl,leftind,ind),leftind)
  else
    (swapTwo(nl,rightind,ind),rightind)

}

//println(heapify(larger, Array(4,1,3,2,16,9,10,14,8,7),3)._1.mkString("-"))

def parentInd(ind:Int) = if (ind < 0) -1 else ((ind - 1) / 2)

def buildHeap(func:(Int,Int)=>Boolean, arr:Array[Int], stack:Array[Int] = Array()):Array[Int] = {
  var stackInner = Array[Int]()
  if (stack.isEmpty) {
    stackInner = stackInner :+ parentInd(arr.length - 1)
  }else{
    stackInner = stack
  }

  if (stackInner(0) == -1)
    arr
  else {
    val temp = heapify(func,arr,stackInner.last)
    val tempStack:Array[Int] = if (temp._2 != null)
      stackInner :+ temp._2.asInstanceOf[Int]
    else Array(stackInner(0) - 1)
    buildHeap(func, temp._1, tempStack)
  }

}

print(buildHeap(larger, Array(4,1,3,2,16,9,10,14,8,7)).mkString("-"))
