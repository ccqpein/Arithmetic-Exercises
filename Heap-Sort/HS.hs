data Heap a = Heap {arr :: [a]} deriving (Eq,Ord)
type Index = Int
data Node a = Empty | Node {val::a,
                            index::Index}
                      deriving (Eq,Ord)

(>@) :: (Ord a) => a -> Node a -> Bool
(>@) i x
  | x == Empty = True
  | otherwise = i > (val x)

(>@@) :: (Ord a) => Node a -> Node a -> Bool
(>@@) i x
  |i == Empty && x == Empty = False
  |x == Empty = True
  |i == Empty = False
  |otherwise = (val i) > (val x)

leftChild :: Heap a -> Index -> Node a -- (val index)
leftChild h i = let ii = 2 * i + 1 in
  if ii <= length (arr h)
  then Node ((arr h)!!ii) ii
  else Empty

rightChild :: Heap a -> Index -> Node a
rightChild h i = let ii = 2 * i + 2 in
  if ii <= length (arr h) - 1
  then Node ((arr h)!!ii) ii
  else Empty

root :: Heap a -> Index -> Node a
root h i = let ii = div i 2 in
             Node ((arr h)!!ii) ii

swapTwo :: Int -> Int -> [a] -> [a]
swapTwo f s xs = map snd . foldr (\x a ->
        if fst x == f then ys !! s : a
        else if fst x == s then ys !! f : a
        else x : a) [] $ ys
    where ys = zip [0..] xs

maxHeapify :: (Ord a) => Heap a -> Index -> Heap a -- heap startIndex
maxHeapify h i
  |rightChild h i == Empty && leftChild h i == Empty = h
  |(arr h)!!i >@ rightChild h i && (arr h)!!i >@ leftChild h i
  = h
  |leftChild h i >@@ rightChild h i
  = maxHeapify (Heap (swapTwo i (index $ leftChild h i) (arr h))) (index $ leftChild h i)
  |rightChild h i >@@ leftChild h i
  = maxHeapify (Heap (swapTwo i (index $ rightChild h i) (arr h))) (index $ rightChild h i)

buildMaxHeap :: (Ord a) => Heap a -> Heap a
buildMaxHeap x = buildMaxHeap' x ((length.arr) x - 1) where
  buildMaxHeap' x len
    | len == 0 = x
    | otherwise = buildMaxHeap' (maxHeapify x $ index (root x len)) (len - 1)
