cutList :: [a] -> ([a],[a])
cutList x = splitAt (div (length x) 2) x

binarySearch :: (Ord a, Num a) => [a] -> a -> Bool
binarySearch [] _ = False
binarySearch ls a
  | head ls == a = True
  | otherwise =
    let (x,y) = cutList ls in
      if head y > a then binarySearch x a
      else binarySearch x a

main = do
  let test = [10,14,19,26,27,31,33,35,42,44]
  print $ binarySearch test 9
  print $ binarySearch test 45
  print $ binarySearch test 29

  print $ binarySearch test 10
  print $ binarySearch test 14
  print $ binarySearch test 31
  print $ binarySearch test 44
