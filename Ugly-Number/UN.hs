isUgly :: Int -> Bool
isUgly 0 = False
isUgly a
  | a `elem` [1,2,3,5] = True
  | a `mod` 2 == 0 = isUgly (a `div` 2)
  | a `mod` 3 == 0 = isUgly (a `div` 3)
  | a `mod` 5 == 0 = isUgly (a `div` 5)
  | otherwise = False

