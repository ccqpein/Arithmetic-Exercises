(defun tribonacci (n)
  (loop
     with init = '(1 1 0)
     for i from 3 to n
     do (setf init (sum-first-three init))
     finally (return (car init))))

(defun sum-first-three (l)
  (cons (apply #'+ (subseq l 0 3)) l))
