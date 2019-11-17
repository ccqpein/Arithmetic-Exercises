rob :: [Int] -> Int
rob [] = 0
rob a
  | length a <= 3 = maximum a
  | otherwise = max (inner_max (tail a) (0, 0)) (inner_max (init a) (0, 0))
  where
    inner_max :: [Int] -> (Int, Int) -> Int
    inner_max [] (_,b) = b
    inner_max (x:xs) (before, now) = inner_max xs (now, (max (x + before) now))

main = do
  print $ rob [1,2,3,1]
  print $ rob [2,7,9,3,1]
