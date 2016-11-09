test = [5,2,6,1]

smallRight :: (Num a, Ord a) => [a] -> [a]
smallRight [] = []
smallRight (n:ns)= (foldl (\acc x -> if x < n then (+1) acc else acc) 0 ns): (smallRight ns)
