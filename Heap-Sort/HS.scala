def heapify[T](func:(T,T) => Boolean,a:T,b:T) = func(a, b)

heapify((a:Int,b:Int)=> if (a > b) true else false ,1,2)
