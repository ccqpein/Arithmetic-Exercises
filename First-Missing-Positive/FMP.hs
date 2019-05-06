import           Data.List (sort)

-- assume [Int] already sorted
first_missing_positive :: [Int] -> Int
first_missing_positive a = inner_function a 1
  where
  inner_function :: [Int] -> Int -> Int
  inner_function [] count = count
  inner_function a count
    | head a <= 0 = inner_function (tail a) count
    | count /= head a = if (count - 1) == (head a)
                        then inner_function (tail a) count
                        else count
    |otherwise = inner_function (tail a) (1 + count)

main = do
  print $ first_missing_positive (sort [1,2,0])
  print $ first_missing_positive (sort [3,4,-1,1])
  print $ first_missing_positive (sort [7,8,9,11,12])
  print $ first_missing_positive (sort [0,2,2,1,1])

