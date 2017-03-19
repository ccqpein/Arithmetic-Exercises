data NestList a = EList |Justt a|NList [(NestList a)]
                               deriving (Show,Eq)


nestList :: (Eq a) => ([NestList a], [NestList a]) -> ([NestList a], [NestList a])
nestList (l, []) = (l, [])
nestList (l, (x:xs)) = case x of
  EList -> nestList (l,xs)
  Justt m -> nestList (l++[x],xs)
  NList m -> nestList ((fst (nestList (l, m))),
                        xs)

main = do
  let testList = [Justt 1,
                   NList [Justt 3, NList [Justt 2]],
                   NList [NList [NList [Justt 5]]]]
  let testList2 = [Justt 1,
                   NList [Justt 3, NList [Justt 2], NList [Justt 6, Justt 7]],
                   NList [NList [Justt 4, NList [Justt 5]]]]
  let resultL = []
  print (fst $ nestList (resultL,testList))
  print (fst $ nestList (resultL,testList2))
