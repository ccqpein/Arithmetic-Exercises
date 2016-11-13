filter' :: [Int] -> [Int]
filter' []       = []
filter' (_:s:ts) = s:(filter' ts)
filter' (_:[])   = []

lastReminding :: [Int] -> Int
lastReminding nL
  | length nL == 1 = head nL
  | otherwise = (lastReminding.reverse.filter') nL
