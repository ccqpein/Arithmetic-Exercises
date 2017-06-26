def quickSort(ls:List[Int]):List[Int] = {
  if (ls.isEmpty)
    List()
  else {
    val key = ls.head
    quickSort(ls.tail.filter(_ <= key)) :::
    key +:
    quickSort(ls.tail.filter(_ > key))
  }

}

print(quickSort(List(2,3,4,5,2,1,4,5,3)).mkString("-"))
