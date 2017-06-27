import           Data.List (sort)

main = do
  let test = [[2],[3,4],[6,5,7],[4,1,8,3]]
  print $ minimumTotal 0 test
  print $ minimumTotal 0 [[2]]


minimumTotal :: Integer -> [[Integer]] -> Integer
minimumTotal = foldl (\a b -> (head.sort) $ map (+ a) b)
