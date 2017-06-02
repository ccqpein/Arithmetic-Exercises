(defun distributeCandies (candies)
  (let ((resultDic (make-hash-table))
        (len (length candies)))
    (loop for i in candies
       do (let ((thisVal (gethash i resultDic)))
            (if (eql nil thisVal)
                (setf (gethash i resultDic) 1)
                (setf (gethash i resultDic) (1+ thisVal)))
            ))
    (let ((lenDic (hash-table-count resultDic)))
      (if (> lenDic (/ len 2))
          (first (multiple-value-list (floor (/ len 2))))
          lenDic))))