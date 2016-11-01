permutations' :: (Fractional a, Enum a, Num a) => a -> a -> a
permutations' a b = let aa = a-b+1 in
  (product [aa..a])/ (product [1..b])

genList :: (Fractional a, Enum a, Num a, Integral a) => a -> [a]
genList a = [1..(div a 2)]

climbS :: (Fractional a, Enum a, Num a, Integral a) => a -> a
climbS a = let llist = (genList a) in
  sum [permutations' a n | n <- llist]
