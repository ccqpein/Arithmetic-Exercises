import           Data.List (findIndex)

length_of_longest_substring :: String -> Int
length_of_longest_substring s =
  let cache = []
   in loop_inner_function cache s 0
  where
    loop_inner_function cache [] largest
      | length cache > largest = length cache
      | otherwise = largest
    loop_inner_function cache (x:xs) largest
      | elem x cache =
        let Just ind = (findIndex (== x) cache)
            large = if length cache > largest then length cache else largest
         in loop_inner_function ((drop (1 + ind) cache) ++ [x]) xs large
      | otherwise = loop_inner_function (cache ++ [x]) xs largest

main = do
  print $ length_of_longest_substring "abcabcb"
  print $ length_of_longest_substring "bbbbb"
  print $ length_of_longest_substring "pwwkew"
  print $ length_of_longest_substring "dvdf"

