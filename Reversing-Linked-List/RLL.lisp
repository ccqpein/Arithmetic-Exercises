(defun reversing-list (l k)
  (if (< (length l) k)
      l
      (let ((head (reverse (subseq l 0 k))))
        (append head (reversing-list (subseq l k) k)))))

(defun main ())
