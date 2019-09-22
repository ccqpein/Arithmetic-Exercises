import           Data.List

make_sort_func :: [Char] -> Char -> Char -> Ordering
make_sort_func s a b = case (elemIndex a s, elemIndex b s) of
  (Nothing, Nothing) -> EQ
  (Just _, Nothing)  -> LT
  (Nothing, Just _)  -> GT
  (Just a_index, Just b_index)
    | a_index >= b_index -> GT
    | otherwise -> LT

custom_sort_string :: [Char] -> [Char] -> [Char]
custom_sort_string s t = sortBy (make_sort_func s) t

main = do
  print $ custom_sort_string "cba" "abcd"
