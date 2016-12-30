(defvar *t1 '(
              (2)
              (3 4)
              (6 5 7)
              (4 1 8 3)))
(defvar *t2 '(
              (2)))

(defun minimum-total (triangle)
  (do ((reL (car triangle))
       (lastrow (cdr triangle) (cdr lastrow)))
      ((eql 'nil lastrow) (car reL))
    (setf reL
          (sort (loop with tempL = '()
                   for i in reL
                   do (loop for ii in (car lastrow)
                         do (push (+ i ii) tempL))
                   finally (return tempL)) #'< ))))
