import           Data.List

smallest_range_ii :: [Int] -> Int -> Int
smallest_range_ii a k
  | length a < 2 = 0
  | otherwise =
    let sorted_a = sort a
        mi = sorted_a !! 0
        ma = sorted_a !! ((length sorted_a) - 1)
     in inner_loop
          sorted_a
          ((ma - mi), ma, mi, k)

inner_loop :: [Int] -> (Int,Int,Int,Int) -> Int
inner_loop (x:[]) (dis,_,_,_) = dis
inner_loop (a:b:xs) (dis, ma, mi, k) =
  inner_loop
    (b : xs)
    (min dis ((max (ma - k) (a + k)) - (min (mi + k) (b - k))), ma, mi, k)

main = do
  print $ smallest_range_ii [1] 0
  print $ smallest_range_ii [0, 10] 2
  print $ smallest_range_ii [1, 3, 6] 3
