moveZero :: [Int] -> [Int]
moveZero = foldr (\x acc -> if x /= 0
                            then x:acc
                            else acc ++ [x]) []
