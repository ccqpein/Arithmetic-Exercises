import qualified Data.Map.Lazy as M

mm = M.empty

hashAdd1 :: (Ord k, Num a) => k -> M.Map k a -> M.Map k a
hashAdd1 k m
  | M.member k m = M.insertWith (+) k 1 m
  | otherwise = M.insert k 1 m
