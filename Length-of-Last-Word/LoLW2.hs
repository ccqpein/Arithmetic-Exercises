import           Data.List.Split

lolw :: String -> Int
lolw = lastLen.(splitOn " ")
  where lastLen xs
          | length xs == 0 = 0
          | (length.last) xs == 0 = lastLen $ init xs
          | otherwise = length $ last xs
