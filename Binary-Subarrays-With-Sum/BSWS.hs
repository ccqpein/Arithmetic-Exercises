import           Data.List as DL

num_subarrays_with_sum :: [Integer] -> Integer -> Integer
num_subarrays_with_sum l 0 =
  let store_ind =
        [-1] ++ (find_all_index_in_list l 1 [-1]) ++ [(toInteger (length l))]
   in inner_func store_ind 0
  where
    inner_func :: [Integer] -> Integer -> Integer
    inner_func (x:y:xs) sum =
      let a = y - x - 1
       in inner_func (y : xs) (sum + (div (a * (a + 1)) 2))
    inner_func (x:[]) sum = sum
num_subarrays_with_sum l s =
  let store_ind =
        [-1] ++ (find_all_index_in_list l 1 [-1]) ++ [(toInteger (length l))]
   in inner_func store_ind 1 s 0
  where
    inner_func :: [Integer] -> Integer -> Integer -> Integer -> Integer
    inner_func l this s sum
      | this <= ((toInteger (length l)) - s - 1) =
        let a = this + s - 1
         in inner_func
              l
              (this + 1)
              s
              (sum +
               (((l !! (fromInteger this)) - (l !! ((fromInteger this) - 1))) *
                ((l !! (fromInteger (a + 1))) - (l !! (fromInteger a)))))
      | otherwise = sum

find_all_index_in_list :: (Eq a) => [a] -> a -> [Integer] -> [Integer]
find_all_index_in_list [] _ cache = tail cache
find_all_index_in_list l a cache =
  case DL.findIndex (== a) l of
    Just ind ->
      find_all_index_in_list
        (tail (snd (splitAt ind l)))
        a
        (cache ++ [((last cache) + (toInteger ind) + 1)])
    Nothing -> tail cache
