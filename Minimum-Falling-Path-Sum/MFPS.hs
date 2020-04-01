min_falling_path_sum :: [[Int]] -> Int
min_falling_path_sum (x:xs) = inner_func x xs
  where
    inner_func l []     = minimum l
    inner_func l (x:xs) = inner_func (sum_together l x) xs
    sum_together a b =
      [ (b !! ind) +
      (minimum
         [ (a !! (max (ind - 1) 0))
         , (a !! ind)
         , (a !! (min (ind + 1) (length b - 1)))
         ])
      | ind <- [0 .. ((length b) - 1)]
      ]


main = do
  print $ min_falling_path_sum [[1,2,3],[4,5,6],[7,8,9]]
