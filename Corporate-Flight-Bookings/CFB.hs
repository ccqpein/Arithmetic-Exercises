corp_flight_bookings :: [[Int]] -> Int -> [Int]
corp_flight_bookings [] _ = []
corp_flight_bookings [[]] _ = []
corp_flight_bookings v n = let result = [0 | _ <- [0..20000]] in
  fst $ splitAt n $ change_vec result v
  where
    change_vec :: [Int] -> [[Int]] -> [Int]
    change_vec re []     = re
    change_vec re (x:xs) = change_vec (sum_list re x) xs

sum_list :: [Int] -> [Int] -> [Int]
sum_list re (a:b:c:_) =
  let (f, s, t) = splitAtDouble (a - 1, b - 1) re
   in f ++ (map (+ c) s) ++ t

splitAtDouble :: (Int,Int) -> [a] -> ([a], [a], [a])
splitAtDouble (f, s) a =
  let (first, tailP) = splitAt f a
      (second, third) = splitAt (s - f + 1) tailP
   in (first, second, third)

main = do
  print $ corp_flight_bookings [[1,2,10], [2,3,20], [2,5,25]] 5
