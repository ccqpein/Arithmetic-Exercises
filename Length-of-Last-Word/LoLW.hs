import           Data.List.Split
splitStr :: String -> [String]
splitStr = splitOn " "

getNum :: [String] -> Int
getNum (s:sL)
  | s == "" = getNum sL
  | otherwise = length s


lolw :: String -> Int
lolw = (getNum.reverse.splitStr)
