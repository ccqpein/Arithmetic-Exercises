data Node = Empty
          | Node { value         :: Int
                 , nodeL         :: [Node]
                 , root          :: Node
                 , numberOfLevel :: Int
                 } deriving (Show, Eq)

findRoot :: Node -> Node
findRoot a
  | root a == Empty = a
  | otherwise = findRoot $ root a

connected :: Node -> Node -> Node
connected a b
  | (numberOfLevel.root) a == (numberOfLevel.root) b = 
