min_add_to_make_valid :: [Char] -> Int
min_add_to_make_valid s = loop_with (0, 0) s
  where
    loop_with :: (Int, Int) -> [Char] -> Int
    loop_with (cache,result ) [] = result + cache
    loop_with (cache, result) (x:xs)
      | x == '(' = loop_with ((cache + 1), result) xs
      | x == ')' =
        loop_with
          (if cache == 0
             then (0, (1 + result))
             else (cache - 1, result)) xs
      |otherwise = 0


main = do
  print $ min_add_to_make_valid "())"
