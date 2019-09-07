(defun num-tree (n)
  (let ((table (make-hash-table)))
    (setf (gethash 0 table) 1
          (gethash 1 table) 1)
    (loop
       for i from 2 to n
       do (loop
             for j from 0 to (- i 1)
             do (setf (gethash i table)
                      (+ (gethash i table 0)
                         (* (gethash j table)
                            (gethash (- i j 1) table))))))
    (gethash n table)))
