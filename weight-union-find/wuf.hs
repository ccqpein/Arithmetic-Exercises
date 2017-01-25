import           Data.Tree

connect :: Tree a -> Tree a -> Tree a
connect a b = Node (rootLabel a) (b:(subForest a))

main = do
  let n0 = Node 0 []
  let n1 = Node 1 []
  let n2 = Node 2 []

  let n3 = connect n0 n1
  let n4 = connect n3 n2

  return n4
