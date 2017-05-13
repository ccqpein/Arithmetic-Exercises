(defun min-distance (tree squirrel nuts)
  (labels ((dist (p1 p2)
             (loop for x in p1
                for y in p2
                sum (abs (- x y)))))
    (let* ((minNutDist (- (dist (car nuts) squirrel)
                          (dist (car nuts) tree))))
      (+ (loop for nut in nuts
            when (<= (- (dist nut squirrel)
                        (dist nut tree)) minNutDist)
            do (setf minNutDist
                     (- (dist nut squirrel)
                        (dist nut tree)))
            sum (* 2 (dist nut tree))
              ) minNutDist))))
