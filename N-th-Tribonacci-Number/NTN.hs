tribonacci :: Int -> Int
tribonacci n =
  let initList = [1,1,0] in
    head $ foldl (\x _ -> ((sum $ take 3 x) : x)) initList [3..n]
