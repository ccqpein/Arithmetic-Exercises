{-# LANGUAGE TypeOperators #-}

sum_mod_together :: [Int] -> Int -> [Int]
sum_mod_together [] _ = [0]
sum_mod_together a k = foldl (add_mod_num k) [0] a
  where
    add_mod_num :: Int -> [Int] -> Int -> [Int]
    add_mod_num _ [] _ = []
    add_mod_num a b c  = b ++ [(((((last b)+ c) `mod` a) + a) `mod` a)]

filter_count :: Int -> [Int] -> [Int]
filter_count k a =
  map (\x -> x * (x - 1) `div` 2) $
  filter (/= 0) [length $ filter (== d) a | d <- [0 .. k]]

subarrays_div_by_k :: [Int] -> Int -> Int
subarrays_div_by_k [] _ = 0
subarrays_div_by_k a k  = sum $ filter_count k $ sum_mod_together a k

main = do
  print $ subarrays_div_by_k [4, 5, 0, -2, -3, 1] 5
  print $ subarrays_div_by_k [-1, 2, 9] 2
  print $ subarrays_div_by_k [2, -2, 2, -4] 6
