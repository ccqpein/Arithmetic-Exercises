(defun min-rectangles-to-cover-points (points w)
  (let ((all-x (sort (mapcar #'car points) #'<))
        (result 1))
    (reduce (lambda (s x)
              (if (> (- x s) w)
                  (progn (incf result)
                         x)
                  s))
            all-x
            :initial-value (car all-x)
            )
    result
    ))

(defun main ()
  (assert (= 2 (min-rectangles-to-cover-points '((2 1) (1 0) (1 4) (1 8) (3 5) (4 6)) 1))))
