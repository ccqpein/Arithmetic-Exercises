(defun add-digits (n)
  (do ((numL (number-to-list n) (number-to-list
				 (apply #'+ numL))))
      ((= (length numL) 1) (format t "~S" (first numL)))))

(defun number-to-list (n)
    (loop for c across (write-to-string n) collect (digit-char-p c)))
