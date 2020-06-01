summary_ranges [] (start,end) = ((make_string start end), [])
summary_ranges (x:xs) (start, end)
  | x > end + 1 = ((make_string start end), (x : xs)) -- this string and new start and end
  | otherwise = summary_ranges xs (start, x)

make_string :: Int -> Int -> String
make_string a b
  | a == b = show a
  | otherwise = (show a) ++ "->" ++ (show b)

loop_body :: [Int] -> IO [String]
loop_body t_case = do
  let (ss, ll) = summary_ranges t_case ((head t_case), (head t_case))
  if ll == []
    then return $ [ss]
    else fmap ((++) [ss]) $ loop_body ll

main = do
  -- print $ summary_ranges [0, 1, 2, 4] (0, 0)
  -- print $ summary_ranges [4] (4, 4)
  let test_case0 = [0, 1, 2, 4, 5, 7]
  let test_case1 = [0, 2, 3, 4, 6, 8, 9]
  loop_body test_case0 >>= print
  loop_body test_case1 >>= print

