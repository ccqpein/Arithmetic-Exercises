addDigits :: Int -> Int
addDigits num
  | num < 0 = -1
  | num `elem` [0..9] = num
  | num >= 10 = addDigits (sum (sepa num))
  where sepa num = map (\x -> read [x]::Int) (show num)
