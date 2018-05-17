--quickSort :: (Ord a) => [a] -> [a]
quickSort [] =[]
quickSort (x:y) = (quickSort $ filter (< x) y) ++ x:(quickSort $ filter (>= x) y)

main = do
  print $ quickSort [2,3,2,1,4,5,6,7,8,5,4,2,1,3,5,6,3,2,1,3,5,3]
