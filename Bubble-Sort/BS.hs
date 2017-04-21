main = do
  let testData = [14,33,27,35,10]
  let flag = False
  let sort a f =
        let temp = bubbleSort a f in
            if snd temp
              then sort (fst temp) flag
              else return (fst temp)
  sort testData flag

bubbleSort :: [Integer] -> Bool -> ([Integer],Bool)
bubbleSort (x:[]) a = ([x],a)
bubbleSort (y:x:xs) a
  | x < y = (x:(fst $ bubbleSort (y:xs) True), True)
  | otherwise = let childResult = bubbleSort (x:xs) a in
                  (y:(fst $ childResult), (snd $ childResult))
