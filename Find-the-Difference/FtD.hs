import qualified Data.Map.Lazy as M

hashAdd1 :: (Ord k, Num a) => k -> M.Map k a -> M.Map k a
hashAdd1 k m
  | M.member k m = M.insertWith (+) k 1 m
  | otherwise = M.insert k 1 m

makeFTDhash :: (Ord k, Num a) => [k] -> M.Map k a -> M.Map k a
makeFTDhash [] m     = m
makeFTDhash (k:ks) m = makeFTDhash ks (hashAdd1 k m)

diff :: (Ord k, Eq a) => [k] -> M.Map k a -> M.Map k a -> k
diff (k:ks) m mm
  |(lookup k $ M.toList m) /= (lookup k $ M.toList mm) = k
  |otherwise = diff ks m mm

findTheDiff :: (Ord k) => [k] -> [k] -> k
findTheDiff s t  = diff (M.keys $ makeFTDhash t M.empty) (makeFTDhash s M.empty) (makeFTDhash t M.empty)
