import qualified Data.Map.Lazy as Map

makeMap :: Ord a => [a] -> [a] -> Map.Map a a
makeMap ppid pid =
  | length x /= length y = Map.empty
  | otherwise = let emptyMap = Map.empty in
      insertPMap emptyMap 0 ppid pid where
      insertPMap m i pp p =
        | i == length ppid = m
        | otherwise = insertPMap (Map.insertWith (++) pp!!i p!!i m) (1 + i) pp p

main = do
  let dict = makeMap [3,0,5,3] [1,3,10,5]
  print dict
