import           Data.List (sort, sortOn)

fkce :: (Num a, Ord a) => [a] -> Int -> a -> [a]
fkce arr@(_:xs) k x
   | last (take (k+1) arr) >= x =
       let temp_result = take (2*k) arr
       in
         sort $ take k (sortOn (\v -> abs (v - x)) temp_result)
   | otherwise = fkce xs k x


main = do
  print (fkce [1,2,3,4,5] 4 3)
  print (fkce [0,0,0,1,3,5,6,7,8,8] 2 2)
