import           Data.List (sort)

sort_array_by_parity_ii :: [Int] -> [Int]
sort_array_by_parity_ii ll =
  let even_l = filter even ll
      odd_l = filter odd ll
  in
    merge_ll (sort even_l) (sort odd_l)
  where
    merge_ll [] y          = y
    merge_ll x []          = x
    merge_ll (x:xs) (y:ys) = x:y:(merge_ll xs ys)

main = do
  print $ sort_array_by_parity_ii [4,2,5,7]
