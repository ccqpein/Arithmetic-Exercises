def replaceWords(dict:Array[String], sentence:String):String = {
  var tempList = Array[String]()
  var sentList = sentence.split(" ")
  for (keyword <- dict) {
    tempList = Array()
    for (word <- sentList) {
      if ((keyword.length <= word.length) && (keyword == word.slice(0,keyword.length))){
        tempList = tempList :+ keyword
      }else {
        tempList = tempList :+ word
      }
    }
    sentList = tempList
  }
  sentList.mkString(" ")
}

println(replaceWords(Array("cat", "bat", "rat"), "the cattle was rattled by the battery"))
