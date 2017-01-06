diffHam :: (Integral a) => a -> a -> a
diffHam a b
  | a == 0 && b == 0 = 0
  | otherwise = if (mod a 2) /= (mod b 2)
                then 1 + diffHam (div a 2) (div b 2)
                else 0 + diffHam (div a 2) (div b 2)

