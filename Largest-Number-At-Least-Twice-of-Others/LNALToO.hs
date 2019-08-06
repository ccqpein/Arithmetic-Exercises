dominantIndex :: [Integer] -> Integer
dominantIndex [] = -1
dominantIndex x = inner_func (0, -1) x
  where
    inner_func :: (Largest_Value, Largest_Ind) -> [Integer] -> Integer
    inner_func 

main = do 
