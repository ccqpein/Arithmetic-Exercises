import qualified Data.Map.Lazy as Map

makeMap :: Ord a => [a] -> [a] -> Map.Map a a
makeMap x y
  | length x /= length y = Map.empty
  | otherwise = Map.fromList $ zip x y


main = do
  let dict = makeMap [3,0,5,3] [1,3,10,5]
  print dict
