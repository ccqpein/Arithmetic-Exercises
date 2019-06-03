{-# LANGUAGE MultiWayIf #-}

import           Data.List

lcs_out :: [Int] -> Int
lcs_out [] = 0
lcs_out a =
  let sorted_l = sort a
      first = head sorted_l in
    lcs (tail sorted_l) (first,1,1)

lcs :: [Int] -> (Int,Int,Int) -> Int
lcs [] (_,largest,_)                   = largest
lcs (x:xs) (last, largest, this_roll)
  | x - last == 1 = if
      | this_roll + 1 > largest -> lcs xs (x, this_roll + 1, this_roll + 1)
      | otherwise -> lcs xs (x, largest, this_roll + 1)
  | x - last == 0 = lcs xs (x, largest, this_roll)
  | otherwise = lcs xs (x, largest, 0)

main = do
  print $ lcs_out [100,4,200,1,3,2]
  print $ lcs_out [0]
  print $ lcs_out [1,2,0,1]
