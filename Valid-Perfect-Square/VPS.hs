-- do not need this
filteWhenFirst :: (Num a) => (a -> Bool) -> [a] -> a
filteWhenFirst f [] =  0
filteWhenFirst f (x:xs) = if f x
                      then x
                      else filteWhenFirst f xs

sumUntil :: (Num a) => (a -> Bool) -> [a] -> a
sumUntil f (x:y:xs) = if f x
                      then x
                      else sumUntil f ((x + y):xs)

isPerfectSquare :: (Num a, Eq a, Ord a, Enum a) => a -> Bool
isPerfectSquare x = if x == (sumUntil (>= x) [1,3..])
                    then True
                    else False

main = do
  let test0 = 14
  let test1 = 16
  print $ isPerfectSquare test0
  print $ isPerfectSquare test1
