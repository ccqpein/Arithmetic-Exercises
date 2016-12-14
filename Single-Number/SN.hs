import qualified Data.Map.Lazy as M


makeTable :: Int -> M.Map Int Int -> M.Map Int Int
makeTable a hs
  | M.member a hs = M.update (\x -> Just (x+1)) a hs
  | otherwise = M.insert a 1 hs

makeL2T :: [Int] -> M.Map Int Int -> M.Map Int Int
makeL2T [] hs     = hs
makeL2T (x:xs) hs = makeTable x (makeL2T xs hs)


