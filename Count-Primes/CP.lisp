(defun count-prime (n)
  (cond ((< n 2) 0)
        (t (let ((bucket (make-list n :initial-element 1)))
             (setf (nth 0 bucket) 0
                   (nth 1 bucket) 0)
             (loop for i from 2 below n
                   if (= 1 (nth i bucket))
                     do (loop for j from (* i i) below n by i
                              do (setf (nth j bucket) 0))
                   end
                   finally (return (apply #'+ bucket)))))))

(defun count-prime2 (n)
  (cond ((< n 2) 0)
        (t (let ((count 0)
                 (table (make-hash-table)))
             (loop for i from 2 below n
                   if (not (gethash i table))
                     do (incf count)
                     and do (loop for a from (* i i) to n by i do (setf (gethash a table) t)))
             count))))

(defun main ()
  (assert (= 4 (count-prime 10)))
  (assert (= 0 (count-prime 1)))
  (assert (= 1229 (count-prime 10000)))
  ;;(assert (= 348513 (count-prime 5000000)))
  ;;(assert (= 0 (count-prime 0)))
  (count-prime 5000000)
  (count-prime2 5000000)
  )
