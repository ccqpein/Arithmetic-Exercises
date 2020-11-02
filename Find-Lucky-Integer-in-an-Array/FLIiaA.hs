import           Data.List

-- copy from https://stackoverflow.com/a/13517443/4493361
frequency s = map (\x -> ([head x], length x)) . group . sort $ s

findLucky :: [Int] -> Int
findLucky [] = -1
findLucky x =
  case find (\(a, b) -> head a == b) $ reverse $ frequency x of
    Just ([a], _) -> a
    Nothing       -> -1

main :: IO ()
main = do
  print (2 == findLucky [2, 2, 3, 4])
  print (3 == findLucky [1, 2, 2, 3, 3, 3])
  print (-1 == findLucky [2, 2, 2, 3, 3])
  print (-1 == findLucky [5])
  print (7 == findLucky [7, 7, 7, 7, 7, 7, 7])
