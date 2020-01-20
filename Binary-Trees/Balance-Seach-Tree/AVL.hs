data Tree a = Empty | Node {this::a,
                           left::Tree a,
                           right::Tree a
                           } deriving (Eq,Show,Ord)


addLeft :: (Eq a) => Tree a -> Tree a -> Tree a
addLeft (Node a l r) y
  |l == Empty = Node a y r
  |otherwise = Node a (addLeft l y) r


addRight :: (Eq a) => Tree a -> Tree a -> Tree a
addRight (Node a l r) y
  |r == Empty = Node a l y
  |otherwise = Node a l (addRight r y)


lengthTree :: (Eq a, Ord a) => Tree a -> Int
lengthTree x
  |x == Empty = 0
  |left x /= Empty || right x /= Empty
  = max (1 + (lengthTree.left) x) (1 + (lengthTree.right) x)
  |otherwise = 0


swapToLeft :: (Eq a) => Tree a -> Tree a
swapToLeft (Node a l r)
  |l == Empty && r /= Empty && (left r) == Empty
  = Node (this r) (Node a Empty Empty) (right r)
  |otherwise = Node a (swapToLeft l) r


swapToRight :: (Eq a) => Tree a -> Tree a
swapToRight (Node a l r)
  |r == Empty && l /= Empty && (right l) == Empty
  = Node (this l) (left l) (Node a Empty Empty)
  |otherwise = Node a l (swapToRight r)


avlExcute :: (Ord a) => Tree a -> Tree a
avlExcute (Node a l r)
  |(lengthTree l) - (lengthTree r) > 1 && (right l) == Empty
  = Node (this l) ((left.swapToLeft) l) (Node a Empty r)
  |(lengthTree r) - (lengthTree l) > 1 && (left r) == Empty
  = Node (this r) (Node a l Empty) ((right.swapToRight) r)
  |otherwise = (Node a l r)

main = do
  let a = Node 4
          (Node 3
            (Node 2
              (Node 1 Empty Empty)
              Empty)
            Empty)
          (Node 5 Empty Empty)
  print (lengthTree a)
  let b = Node 10
          (Node 5
           (Node 2 Empty (Node 3 Empty Empty))
           Empty)
          (Node 15 Empty Empty)
  --print (lengthTree (left b))
  --print (lengthTree (right b))
  print (avlExcute b)
  print "empty line"
  let c = Node 10
          (Node 5 Empty Empty)
          (Node 15 Empty (Node 20 (Node 18 Empty Empty) Empty))
  --print (lengthTree (right c))
  --print (swapToRight c)
  print (avlExcute c)
