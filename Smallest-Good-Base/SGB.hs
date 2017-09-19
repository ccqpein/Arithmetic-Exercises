-- well, I give up
-- RealFrac is a weird typeclass, or becasue I am still suck with haskell
log' :: (Integral a, RealFrac a, Floating a) => a -> a
log' = floor.(+1).logBase 2

pow' :: (Integral b, RealFrac a, Integral a) => a -> a -> b
pow' n l = floor (n ^ (1 / (l-1)))

checkAnswer :: (Integral b) => b -> b -> b
checkAnswer k l = (k ^ l - 1) `div` (k - 1)


{--
smallGoodBase :: (RealFrac a, Integral a) => a -> a -> a
smallGoodBase n l
  | l <= 2
  = n - 1
  | otherwise
  = let k = (pow' n l) :: Int in
      if checkAnswer k l == n then k
      else smallGoodBase n (l - 1)
--}

{--
main = do
  let test1 = 13
      test2 = 4681
      test3 = 1000000000000000000
  print (smallGoodBase test1)
--}
