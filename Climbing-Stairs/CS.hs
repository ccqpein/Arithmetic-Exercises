permutations' :: (Enum a, Num a) => a -> a -> a
permutations' a b = let aa = a-b+1 in
  (fromRational (product [aa..a]))/ (fromRational (product [1..b]))

climbS :: (Fractional a, Enum a, Num a, Integral a) => a -> a
climbS nn = sum [(permutations' n (n + y))| n <- [1..(div nn 2)], let y = (nn - n * 2)]
