testData = [(1,2),(2,4),(3,4),(3,6),(3,5),(4,7),(4,5),(7,8),(6,9)]


-- merge sort function came from ../Merge-Sort/MS.hs
mergeSort :: [(Integer, Integer)] -> [(Integer, Integer)]
mergeSort x
  |length x >= 3 = let (temp1,temp2) = splitAt (div (length x) 2) x
                   in merge (mergeSort temp1) (mergeSort temp2)
  |otherwise = let (temp1,temp2) = splitAt (div (length x) 2) x in
     merge temp1 temp2
     where merge [] b = b
           merge (x:xs) (y:ys)
             | (snd x) <= (snd y) = x : (merge xs (y:ys))
             | (snd y) < (snd x) = y : (merge ys (x:xs))

-- need sort function

ga :: [(Integer,Integer)] -> [(Integer,Integer)]
ga (x:xs) = foldl (\x y -> if fst y >= (snd.last) x
                           then x ++ [y]
                           else x) [x] xs

main = do
  let dd = testData
  print (mergeSort dd)
  return (ga dd)
