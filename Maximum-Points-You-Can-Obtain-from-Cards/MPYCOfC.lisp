(defun max-score (card-points k)
  (loop
    for i from 0 to k
    for v = (+ (apply #'+ (subseq card-points 0 (- k i)))
               (apply #'+ (subseq card-points
                                  (- (length card-points) i)
                                  (length card-points))))
    maximize v))

(assert (= 2429 (max-score '(53 14 91 35 51 9 80 27 6 15 77 86 34 62 55 45 91 45 23 75 66
                             42 62 13 34 18 89 67 93 83 100 14 92 73 48 2 47 93 99 100 88 84 48)
                 43)))

(assert (= 55 (max-score '(9 7 7 9 7 7 9) 7)))
