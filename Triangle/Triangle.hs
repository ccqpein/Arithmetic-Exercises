import           Data.List (sort)

test = [[2],[3,4],[6,5,7],[4,1,8,3]]

nextRow :: [Integer] -> [Integer] -> [Integer]
nextRow a [] = a
nextRow a b  = [(x + y) | x <- a, y <- b]

minimumTotal :: [[Integer]] -> Integer
minimumTotal []     = 0
minimumTotal (x:xs) = (head.sort)$ foldl nextRow x xs
