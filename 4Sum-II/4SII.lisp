(defun four-sum-count (a b c d)
  (labels ((helper (a b)
             (let ((table (make-hash-table)))
               (loop
                 for aa in a
                 do (loop
                      for bb in b
                      do (setf (gethash (+ aa bb) table)
                               (1+ (gethash (+ aa bb) table 0)))))
               table)))
    (let ((at (helper a b))
          (bt (helper c d)))
      (loop
        for key being the hash-key of at
          using (hash-value v) 
        when (gethash (- 0 key) bt)
          sum (* (gethash (- 0 key) bt) v) into result
        end
        finally (return result)
        )))
  )
