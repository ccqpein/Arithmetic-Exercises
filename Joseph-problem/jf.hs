{-# LANGUAGE BangPatterns #-}
import           Data.List

jf :: Int -> Int
jf k
  | inner (2 * k) k (1, 1) = 1
  | otherwise =
    case find
           (inner (2 * k) k)
           [((x * (1 + k)), (x * (1 + k))) | x <- [1 ..]] of
      Just (_, n) -> n
      Nothing     -> 0 -- never here

inner :: Int -> Int -> (Int, Int) -> Bool
inner !left !k (!flex_m, !m)
  | left == k = True
  | 0 == flex_m `mod` left = inner (left - 1) k (m, m)
  | flex_m `mod` left <= k = False
  | otherwise = inner (left - 1) k ((m - (left - (flex_m `mod` left))), m)

main = do
  print $ jf 15
