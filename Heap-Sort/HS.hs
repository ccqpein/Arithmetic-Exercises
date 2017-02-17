data Heap a = Heap {arr :: [a]}
type Index = Int
data Node a = Empty | Node {val::a,
                            index::Index}
                      deriving (Eq)
  
leftChild :: Heap a -> Index -> Node a -- (val index)
leftChild h i = let ii = 2 * i + 1 in
  if ii <= length (arr h)
  then Node ((arr h)!!ii) ii
  else Empty

rightChild :: Heap a -> Index -> Node a
rightChild h i = let ii = 2 * i + 2 in
  if ii <= length (arr h)
  then Node ((arr h)!!ii) ii
  else Empty

swapTwo :: Int -> Int -> [a] -> [a]
swapTwo f s xs = map snd . foldr (\x a ->
        if fst x == f then ys !! s : a
        else if fst x == s then ys !! f : a
        else x : a) [] $ ys
    where ys = zip [0..] xs

maxHeapify :: (Ord a) => Heap a -> Index -> Heap a -- heap startIndex
maxHeapify h i
  |rightChild h i == Empty || leftChild h i == Empty = h
  |(arr h)!!i > val (rightChild h i) && (arr h)!!i > val (leftChild h i)
  = h
  |(val $ leftChild h i) >= (val $ rightChild h i)
  = maxHeapify (Heap (swapTwo i (index $ leftChild h i) (arr h))) (index $ leftChild h i)
  |(val $ leftChild h i) < (val $ rightChild h i)
  = maxHeapify (Heap (swapTwo i (index $ rightChild h i) (arr h))) (index $ rightChild h i)
