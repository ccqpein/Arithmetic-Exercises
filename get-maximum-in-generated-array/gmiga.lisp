(ql:quickload "alexandria")

(defun get-maximum-generated (n)
  (labels ((helper (n table)
             (alexandria:if-let ((v (gethash n table)))
               v
               (cond ((= n 0) 0)
                     ((= n 1) 1)
                     (t (if (evenp n)
                            (setf (gethash n table) (helper (/ n 2) table))
                            (setf (gethash n table)
                                  (+ (helper (floor (/ n 2)) table)
                                     (helper (ceiling (/ n 2)) table))))
                        (gethash n table))))))
    (let ((table (make-hash-table :test 'equal)))
      (loop for i from 0 to n
            maximize (helper i table)))))

(defun main ()
  (assert (= 3 (get-maximum-generated 7)))
  (assert (= 1 (get-maximum-generated 2)))
  (assert (= 2 (get-maximum-generated 3)))
  (assert (= 2 (get-maximum-generated 4))))
