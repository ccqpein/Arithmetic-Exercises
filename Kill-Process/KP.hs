import qualified Data.Map.Lazy as Map

makeMap :: Ord a => [a] -> [a] -> Map.Map a [a]
makeMap ppid pid
  | length ppid /= length pid = Map.empty
  | otherwise = let emptyMap = Map.empty in
      insertPMap emptyMap 0 ppid pid where
      insertPMap m i pp p
        | i == length ppid = m
        | otherwise = insertPMap (Map.insertWith (++) (pp!!i) [p!!i] m) (1 + i) pp p



killProcess :: (Ord a, Num a) => Map.Map a [a] -> [a] -> [a]
-- I dont know whether Map.empty always be matched
-- killProcess empty _  = [100]
killProcess _ []     = []
killProcess m (x:xs)
  | m Map.!? x == Nothing = (killProcess m xs) ++ [x]
  | otherwise = (killProcess m ((m Map.! x) ++ xs)) ++ [x]

main = do
  let dict = makeMap [3,0,5,3] [1,3,10,5]
  print dict
  -- := Bug next, give 2 which not including in ppid or pid, will return [2]
  print $ killProcess dict [2]
  print $ killProcess dict [3]
  print $ killProcess dict [0]

