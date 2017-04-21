{--main = do
  let testData = [14,33,27,35,10]
  let flag = True
  print (bubbleSort testData flag)
--}
bubbleSort :: ([Integer],Bool) -> ([Integer],Bool)
bubbleSort ((x:[]),a) = ([x],a)
bubbleSort (y:x:xs)
  | x < y = x:(bubbleSort (y:xs))
  | otherwise = y:(bubbleSort (x:xs))
