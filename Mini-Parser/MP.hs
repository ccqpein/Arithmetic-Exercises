data NestedInteger = Empty | NestedInteger Int NestedInteger deriving (Show)

deserialize :: [Int] -> NestedInteger
deserialize []     = Empty
deserialize (x:xs) = NestedInteger x (deserialize xs)

parerIntList :: String -> [Int]
parerIntList ss = read ('[' :[s | s <- ss, s /= '[' && s /= ']'] ++ [']']) :: [Int]

