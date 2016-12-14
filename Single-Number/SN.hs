import qualified Data.Map.Lazy as M

let hs = M.empty

makeTable :: Int -> M.Map Int Int -> M.Map Int Int
makeTable a hs
  | M.member a hs = M.update a ((1+) $ lookup a hs)
  | otherwise = M.insert a 1 hs

