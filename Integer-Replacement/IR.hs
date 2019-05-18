integer_replacement :: Int -> Int
integer_replacement a = inner_func a 0
  where
    inner_func :: Int -> Int -> Int
    inner_func a count
      | a == 1 = count
      | even a = inner_func (div a 2) (1 + count)
      | odd a = min (inner_func (a + 1) (1 + count))
                (inner_func (a - 1) (1 + count))

main = do
  print $ integer_replacement 8
  print $ integer_replacement 7
  print $ integer_replacement 1234
  print $ integer_replacement 2147483647


