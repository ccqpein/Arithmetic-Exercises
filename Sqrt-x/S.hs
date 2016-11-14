mySqrt :: (Eq a, Floating a) => a -> a
mySqrt a
  | a == 0 || a == 1 = a
  |otherwise = sqrt a

mySqrt2 :: (RealFrac a) => a -> Int
mySqrt2 = floor

-- mySqrt2 $ mySqrt 125348
-- Still comfusion of div and type system
