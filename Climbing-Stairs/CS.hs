factorial' :: (Enum a, Eq a, Num a, Fractional a) => a -> a
factorial' n = product [1..n]

permutations' :: (Enum a, Eq a, Num a, Fractional a) => a -> a -> a
permutations' a _ = factorial' a
permutations' a b = (factorial' a) / ((factorial' b) * (factorial' (a-b)))
