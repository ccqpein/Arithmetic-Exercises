data NestedInteger = Empty | NestedInteger Int NestedInteger deriving (Show)

deserialize :: [Int] -> NestedInteger
deserialize []     = Empty
deserialize (x:xs) = NestedInteger x (deserialize xs)

parerIntList :: String -> [Int]
parerIntList = read.clearSymbol where
  clearSymbol = foldl (\acc x ->
                         | (x /= '[') `or` (x /= ']') = acc ++ x
                         |otherwise = '') ""

