test1 = [] -- this test make error, need Maybe data to fix it
test2 = [2,1,2,1,0,1,2]
test3 = [1]
test4 = [7,1,5,3,6,4]
test5 = [2,4,1]

maxProfit :: [Integer] -> Integer
maxProfit (x:xs) = let reL = [0,x,0] in
     maxProfit' xs reL where
     maxProfit' (x:xs) reL = maxProfit' xs (innerMP x reL)
     maxProfit' [] reL     = last reL


-- () = (minv maxv result)
innerMP :: Integer -> [Integer] -> [Integer]
innerMP a (maxv:minv:result)
  | a < minv = [maxv,a] ++ result
  | (a - minv) > (head result) = [a,minv] ++ [(a - minv)]
  | otherwise = [maxv,minv] ++ result
