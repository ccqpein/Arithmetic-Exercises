import           Data.List as DL

-- num_subarrays_with_sum :: [Integer] -> Integer -> Integer
-- num_subarrays_with_sum _ 0 = 0
-- num_subarrays_with_sum l s =
--   let store_ind = [-1] ++ [| v <- l] ++ [(length l)]


find_all_index_in_list :: (Eq a) => [a] -> a -> [Int] -> [Int]
find_all_index_in_list [] _ cache = tail cache
find_all_index_in_list l a cache =
  case DL.findIndex (== a) l of
    Just ind -> find_all_index_in_list (tail (snd (splitAt ind l))) a (cache ++ [((last cache) + ind+ 1)])
    Nothing  -> tail cache
