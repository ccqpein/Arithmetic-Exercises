(defun max-weight (pizzas)
  (let* ((pizzas (sort pizzas #'>))
         (step (floor (/ (length pizzas) 4))))
    (let ((xx (+ (floor (/ step 2)) (mod step 2)))
          (yy (floor (/ step 2))))
      ;; (pprint step)
      ;; (pprint xx)
      ;; (pprint yy)
      (+ (loop for ind from 0 below xx
               sum (nth ind pizzas))
         (loop with rest = (subseq pizzas xx)
               for ind from 0 below yy
               sum (nth (1+ (* ind 2)) rest))))))

(defun main ()
  (assert (= 14 (max-weight '(1 2 3 4 5 6 7 8))))
  (assert (= 3 (max-weight '(2 1 1 1 1 1 1 1))))
  (assert (= 14 (max-weight '(5 2 2 4 3 3 1 3 2 5 4 2)))))
