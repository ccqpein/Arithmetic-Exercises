import qualified Data.Map.Lazy as M

-- hs = M.empty

makeTable :: Int -> M.Map Int Int -> M.Map Int Int
makeTable a hs
  | M.member a hs = M.update (\x -> Just (x+1)) a hs
  | otherwise = M.insert a 1 hs

makeL2T :: [Int] -> M.Map Int Int -> M.Map Int Int
makeL2T [] hs     = hs
makeL2T (x:xs) hs = makeTable x (makeL2T xs hs)

sN :: [Int] -> M.Map Int Int -> Maybe Int --[Int] is [keys]
sN [] _ = Nothing
sN (x:xs) hs
  | M.lookup x hs == Just 1 = Just x
  | otherwise = sN xs hs

{-- Test sample
λ> let hs = M.empty
λ> hs2 = makeL2T [1,1,2,2,3,4,4] hs
λ> sN (M.keys hs2) hs2
Just 3
--}
