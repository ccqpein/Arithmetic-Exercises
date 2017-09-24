checkPerfectNumber :: Int -> Bool
checkPerfectNumber a
  | a <= 1 = False
  | a * 2== foldr (\x z -> if mod a x == 0
                 then z + x + a `div` x
                 else z) 0
    [1..edge a] = True
  | otherwise = False


edge :: Int -> Int
edge a = round (sqrt (fromIntegral a) :: Double) :: Int
