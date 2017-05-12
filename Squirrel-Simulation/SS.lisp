(defun min-distance (tree squirrel nuts)
  (let* ((dist (lambda (p1 p2)
                 (loop for x in p1
                    for y in p2
                    sum (abs (- x y)))))
         (minNutDist (- (dist (car nuts) squirrel)
                        (dist (car nuts) tree)))
         (nuts2tree 0))
    (loop for nut in nuts
         do ())))
