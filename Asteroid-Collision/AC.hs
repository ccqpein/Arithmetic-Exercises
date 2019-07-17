data Collision_Status = NoCollision | LeftLarger | Equal | RightLarger
  deriving (Eq)

two_list_collision :: [a] -> [a] -> (a -> a -> Collision_Status) -> [a]
two_list_collision a [] _      = a
two_list_collision [] (x:xs) f = two_list_collision [x] xs f
two_list_collision a (x:xs) f
  | f (last a) x == NoCollision = two_list_collision (a ++ [x]) xs f
  | f (last a) x == LeftLarger = two_list_collision a xs f
  | f (last a) x == Equal = two_list_collision (init a) xs f
  | f (last a) x == RightLarger = two_list_collision (init a) (x:xs) f

asteroid_collision :: [Int] -> [Int]
asteroid_collision [] = []
asteroid_collision (x:xs) = two_list_collision [x] xs make_collision_function
  where
    make_collision_function :: Int -> Int -> Collision_Status
    make_collision_function a b
      | a > 0 && b < 0 =
        if a > abs b
          then LeftLarger
          else if a == abs b
                 then Equal
                 else RightLarger
      | otherwise = NoCollision

main = do
  print $ asteroid_collision [5, 10, -5]
  print $ asteroid_collision [8, -8]
  print $ asteroid_collision [10, 2, -5]
  print $ asteroid_collision [1, -2, -2, -2]
