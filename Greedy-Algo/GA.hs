testData = [(1,2),(2,4),(3,4),(3,6),(3,5),(4,7),(4,5),(7,8),(6,9)]

-- need sort function

ga :: [(Integer,Integer)] -> [(Integer,Integer)]
ga (x:xs) = foldl (\x y -> if fst y >= (snd.last) x
                           then x ++ [y]
                           else x) [x] xs

main = do
  let dd = testData
  return (ga dd)
