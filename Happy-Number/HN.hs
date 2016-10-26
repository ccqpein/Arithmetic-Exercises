import           Data.Char

split :: String -> [Int]
split []     = []
split (s:sx) = (digitToInt s):(split sx)

isHappy :: Int -> Int
isHappy a
  | a < 1 = 0
  | a == 1 = 1
  | otherwise = isHappy ((sum.(map (^2)).split.show) a)
