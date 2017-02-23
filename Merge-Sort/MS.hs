{--
Haskell complier is GHC (https://haskell.org/ghc/).
If you want to test code, go https://www.jdoodle.com/execute-haskell-online.
Copy and Ececute code
--}

-- input two lists, return one sorted list
merge :: [Int] -> [Int] ->[Int]
-- when first list is empty, return the second list directly
merge [] x = x
-- Deconstruct two list and compare with first elements of two lists
merge (x:xs) (y:ys)
  -- make smaller be first element, recurse left list. Make shorter list be first argument
  | x <= y = x : (merge xs (y:ys))
  | y < x = y : (merge ys (x:xs))


-- input list, return list
mergeSort :: [Int] -> [Int]
mergeSort x
  -- when length of list >= 3, spilit list to two lists and maerge results of recursed spilit two lists
  |length x >= 3 = let (temp1,temp2) = splitAt (div (length x) 2) x
                   in merge (mergeSort temp1) (mergeSort temp2) -- recursion
  -- when length of list < 3, spilit it to two lists, and call merge function sort them
  |otherwise = let (temp1,temp2) = splitAt (div (length x) 2) x in
     merge temp1 temp2

-- main function
main = do
  let b = [3, 41, 52, 26, 38, 57, 9, 49] -- test list
  print (mergeSort b)
