(defun number-of-points (nums)
  (let ((nums (sort nums #'< :key #'car)))
    (do* ((x (first (car nums)))
          (y (second (car nums)))
          (result (- y x -1))
          (rest (cdr nums) (cdr rest))
          (a (car rest) (car rest)))
         ((not a) result)
      (if (<= (first a) y)
          (if (>= (second a) y)
              (setf result (+ result (second a) (- y))
                    y (second a)))
          (setf result (+ result 1 (second a) (- (first a)))
                y (second a))
          )
         )))

(defun main ()
  (assert (= 7 (number-of-points '((3 6) (1 5) (4 7)))))
  (assert (= 7 (number-of-points '((1 3) (5 8)))))
  (assert (= 9 (number-of-points '((2 3) (3 9) (5 7) (4 10) (9 10)))))
  )