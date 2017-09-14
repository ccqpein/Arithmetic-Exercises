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
-- https://stackoverflow.com/questions/46144610/haskell-pattern-matching-match-map-empty-any-with-any-map-map-k-v
killProcess _ []     = []
killProcess m (x:xs)
  | Map.null m = []
  | m Map.!? x == Nothing = (killProcess m xs) ++ [x]
  | otherwise = (killProcess m ((m Map.! x) ++ xs)) ++ [x]

-- Test pattern matching of constructor.
data Foo = Bar | Baz Int
f :: Foo -> Int
f Bar     = 1
f (Baz x) = x - 1

main = do
  let dict = makeMap [3,0,5,3] [1,3,10,5]
  print dict
  -- := Bug next, give 2 which not including in ppid or pid, will return [2]
  print $ killProcess Map.empty [1,2,3]
  print $ killProcess dict [2]
  print $ killProcess dict [3]
  print $ killProcess dict [0]

