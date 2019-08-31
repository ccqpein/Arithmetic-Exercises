import           Data.Char

shifting_letters :: [Char] -> [Int] -> [Char]
shifting_letters s l = inner_shift ([], (reverse s)) (0, (reverse l))

inner_shift :: ([Char], [Char]) -> (Int, [Int]) -> [Char]
inner_shift (result, []) (_, _)               = result
inner_shift (result, _) (_, [])               = result
inner_shift (result, (x:xs)) (before, (y:ys)) =
  let (c, e) = shift_char x (before, y)
   in inner_shift ((c : result), xs) (e, ys)

type Before = Int
type ThisInd = Int

shift_char :: Char -> (Before, ThisInd) -> (Char, Before)
shift_char c (ib, i) =
  let new_sum = mod (ib + i) 26
      this_char = ((ord c) + new_sum)
      x = if this_char > 122 then this_char -26 else this_char
   in (chr x, new_sum)


main = do
  print $ shifting_letters "abc" [3, 5, 9]
  print $ shifting_letters "bad" [10, 20, 30]
  print $
    shifting_letters
      "mkgfzkkuxownxvfvxasy"
      [ 505870226
      , 437526072
      , 266740649
      , 224336793
      , 532917782
      , 311122363
      , 567754492
      , 595798950
      , 81520022
      , 684110326
      , 137742843
      , 275267355
      , 856903962
      , 148291585
      , 919054234
      , 467541837
      , 622939912
      , 116899933
      , 983296461
      , 536563513
      ]
