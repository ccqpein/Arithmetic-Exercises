(defun min-operations (num)
  (let ((table (make-hash-table)))
    (loop for n in num
          do (incf (gethash n table 0)))

    (loop for f being the hash-value of table
          when (= 1 f)
            return -1
          sum (case (mod f 3)
                (0 (/ f 3))
                ((1 2) (1+ (floor (/ f 3))))
                (t (error "")))
          into result
          finally (return result)
          ))) 

(defun main ()
  (assert (= 4 (min-operations '(2 3 3 2 2 4 2 3 4))))
  (assert (= -1 (min-operations '(2 1 2 2 3 3)))))
