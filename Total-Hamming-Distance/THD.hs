diffHam :: (Integral a) => a -> a -> a
diffHam a b
  | a == 0 && b == 0 = 0
  | otherwise = let rr = if (mod a 2) /= (mod b 2)
                      then 1
                      else 0
                in rr + diffHam (div a 2) (div b 2)

totalHammingDistance :: (Integral a) => [a] -> a
totalHammingDistance []     = 0
totalHammingDistance (x:xs) = (sum $ map (diffHam x) xs) + totalHammingDistance xs
