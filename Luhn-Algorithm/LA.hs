import           Data.Char (digitToInt)

luhnIsValid :: [Char] -> Bool
luhnIsValid [] = False
luhnIsValid x = let op_x = (preOperate x) in
  case op_x of
    Nothing -> False
    Just xx -> innerSum xx
  where
    preOperate :: [Char] -> Maybe [Int]
    preOperate = Just . (map digitToInt) . (filter (/= ' ')) -- := error handle

    innerSum :: [Int] -> Bool
    innerSum xs
      | length xs <= 1 = False
      | otherwise = let a = [[x | (x,i)<-zip (reverse xs) [0..], i `mod` 2 == j] | j<-[0..1]] in
                      mod ((sum (a !! 0)) + (sum (map (checkNine . (* 2)) (a !! 1)))) 10 == 0

    checkNine x = if x > 9 then x - 9 else x

main = do
  print $ luhnIsValid "0" -- false
  print $ luhnIsValid "1" -- false
  print $ luhnIsValid "059" -- true
  print $ luhnIsValid "59" -- true
  print $ luhnIsValid "055 444  285" -- true
  print $ luhnIsValid "0000 0" -- true
