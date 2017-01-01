ham :: (Integral a) => a -> a
ham a
  | a == 1 = 1
  | otherwise = (mod a 2) + ham (div a 2)
