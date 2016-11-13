(defun filter-sequence (l)
  (loop for i from 1 to (- (length l) 1) by 2
     collect (nth i l)))

(defun last-remaining (n)
  (let ((l (loop for i from 1 to n collect i)))
    (labels ((temp (ll)
               (if (/= (length ll) 1)
                   (temp (reverse (filter-sequence ll)))
                   (car ll))))
      (temp l))))

