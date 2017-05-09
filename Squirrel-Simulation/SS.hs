type TreeCord = [Integer]
type Squirrel = [Integer]
type Nuts = [[Integer]]

dist :: [Integer] -> [Integer] -> Integer
dist a b = sum$ map (\(a,b) -> abs (a - b)) $ zip a b

minDistance :: TreeCord -> Squirrel -> Nuts -> IO Integer
minDistance tree squirrel nuts = do
  let tempResult = (sum $ map (\x -> 2 * (dist x tree)) nuts,
                    minimum $map (\x -> (dist x squirrel) - (dist x tree)) nuts)
  print (fst tempResult)
  print (snd tempResult)
  return ((fst tempResult) - (snd tempResult))
