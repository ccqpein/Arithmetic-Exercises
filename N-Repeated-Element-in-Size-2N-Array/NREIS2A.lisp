(defun repeat-n-times (l)
  (let ((set (make-hash-table))
        )
    (loop
       for i in l
       if (cadr (multiple-value-list (gethash i set)))
       do (return i)
       else
         do (setf (gethash i set) t))))
