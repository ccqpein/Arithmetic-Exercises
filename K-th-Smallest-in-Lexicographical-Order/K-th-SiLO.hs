find_kth_number :: Int -> Int -> Int
find_kth_number n k = loop_body n 1 (k - 1)
  where
    loop_body :: Int -> Int -> Int -> Int
    loop_body n curr k
      | k <= 0 = curr
      | otherwise =
        let steps = cal_steps n curr (1 + curr)
         in if steps <= k
              then loop_body n (1 + curr) (k - steps)
              else loop_body n (10 * curr) (k - 1)

cal_steps :: Int -> Int -> Int -> Int
cal_steps n n1 n2 =
  give_steps 0 n n1 n2
  where
    give_steps :: Int -> Int -> Int -> Int -> Int
    give_steps steps n n1 n2
      | n1 > n = steps
      | otherwise = give_steps (steps + ((min (1 + n) n2) - n1)) n (10 * n1) (10 * n2)

main = do
  print $ find_kth_number 13 2
  print $ find_kth_number 100 90
  print $ find_kth_number 681692778 351251360

