type Largest_Ind = Integer
type Largest_Value = Integer
type This_ind = Integer

dominantIndex :: [Integer] -> Integer
dominantIndex [] = -1
dominantIndex x = inner_func (0, -1, 0) x
  where
    inner_func :: (Largest_Value, Largest_Ind, This_ind) -> [Integer] -> Integer
    inner_func (_, a, _) [] = a
    inner_func (v, a, t) (x:xs)
      | x >= 2 * v = inner_func (x, t, t + 1) xs
      | x > v = inner_func (x, -1, t + 1) xs
      | 2 * x > v = inner_func (v, -1, t + 1) xs
      | otherwise = inner_func (v, a, t + 1) xs


main = do
  print $ dominantIndex [3,6,1,0]
  print $ dominantIndex [1,2,3,4]
