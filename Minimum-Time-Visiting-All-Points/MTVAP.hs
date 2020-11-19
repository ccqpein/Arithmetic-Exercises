minTime2VisitAllPoints :: (Num a, Ord a, Eq a) => [(a,a)] -> a
minTime2VisitAllPoints points = snd (foldl innerFunc ((head points), 0) points)
  where
    innerFunc :: (Num a, Ord a) => ((a, a), a) -> (a, a) -> ((a, a), a)
    innerFunc ((last0, last1), acc) (x0, x1) =
      ((x0, x1), (acc + (max (abs (x0 - last0)) (abs (x1 - last1)))))

main = do
  print (7 == (minTime2VisitAllPoints [(1, 1), (3,4), (-1,0)]))
  print (5 == (minTime2VisitAllPoints [(3,2), (-2,2)]))
