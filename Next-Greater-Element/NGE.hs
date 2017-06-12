nextGreaterElements :: [Integer] -> [Integer]
nextGreaterElements [] = []
nextGreaterElements nums = let inds = [y | _ <- [0,1],y <- [0..(length nums) - 1]]
                               stack = []
                               res = [-1 | _ <- [0..(length nums) -1]] in
                             innerFunc nums stack inds res where
  innerFunc nums stack inds res
    | null inds = res
    | not (null stack) &&
      nums!!(last stack) < nums!!(head inds)
    = innerFunc nums (init stack) inds $
      take (last stack) res ++ [nums!!(head inds)] ++ drop ((last stack) + 1) res
    | otherwise = innerFunc nums (stack ++ [(head inds)]) (tail inds) res


main = do
  print $ nextGreaterElements [1,2,1]
  print $ nextGreaterElements [5,3,4,6,7]
