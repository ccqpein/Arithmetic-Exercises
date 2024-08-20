(defun nth-ugly-number (n)
  (let ((cache '(1 2 3 4 5)))
    (if (<= n 5) (return-from nth-ugly-number (nth (1- n) cache)))

    (loop for i from 6 to n
          for the-last = (car (last cache))
          for vv =  (apply #'min
                           (remove-if #'(lambda (v) (<= v the-last))
                                      (loop for v in cache
                                            append (list (* 2 v) (* 3 v) (* 5 v)))))
          do (loop if (<= (* 5 (car cache)) vv)
                     do (pop cache)
                   else
                     return nil)
          do (setf cache (append cache (list vv)))
          )
    (car (last cache))
    ))

(defun main ()
  (assert (= 12 (nth-ugly-number 10)))
  (assert (= 1 (nth-ugly-number 1)))
  (assert (= 2123366400 (nth-ugly-number 1690))))
