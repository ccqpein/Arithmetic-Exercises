log' = logBase 2

smallGoodBase :: (Floating a, Eq a, Ord a, RealFrac a, Integral a) => a -> a -> a
smallGoodBase n l
  | l < 2
  = n - 1
  | otherwise
  = let k = round (n ** (1 / (l - 1))) in
      if (k ** l - 1) / (k - 1) == 0
      then k
      else smallGoodBase n (l - 1)

{--
main = do
  let test1 = 13
      test2 = 4681
      test3 = 1000000000000000000
  print (smallGoodBase test1)
--}
