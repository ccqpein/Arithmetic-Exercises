distribution_Candies :: Int -> Int -> [Int]
distribution_Candies c n =
  let result = [0 | _ <- [1 .. n]]
   in inner_dis 1 c (splitAt 0 result)
  where
    inner_dis :: Int -> Int -> ([Int], [Int]) -> [Int]
    inner_dis count can (h, []) = inner_dis count can ([], h)
    inner_dis count can (h, (x:xs))
      | count >= can = h ++ [x + can] ++ xs
      | otherwise = inner_dis (count + 1) (can - count) ((h ++ [x + count]), xs)


main = do
  print $ distribution_Candies 7 4
  print $ distribution_Candies 10 3
