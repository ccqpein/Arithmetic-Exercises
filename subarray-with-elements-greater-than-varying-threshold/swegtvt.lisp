(defparameter *test-cases* '(((1 3 4 3 1) 6 3)
                             ((6 5 6 5 8) 7 5)
                             ((818 232 595 418 608 229 37 330 876 774 931 939 479 884 354 328) 3790 -1)))

(defun valid-subarray-size (nums threshold)
  (let ((cache (loop repeat (length nums) collect `(0 ,(length nums)))))
    (loop for x from 0 below (length nums)
          for xx = (nth x nums)
          do (loop for y from (1+ x) below (length nums)
                   do (cond ((> (nth y nums) xx)
                             (setf (car (nth y cache)) (1+ x)))
                            ((< (nth y nums) xx)
                             (setf (cadr (nth x cache)) y)
                             (return))
                            ))
          if (< (/ threshold (apply #'- (reverse (nth x cache)))) (nth x nums))
             do (return-from valid-subarray-size (apply #'- (reverse (nth x cache)))))
    -1
    ))

(defun main ()
  (loop for x in *test-cases*
        do (assert (= (valid-subarray-size (first x) (second x)) (third x)))))
