;; tree: '(1 (2 (3 (4))))

(defun swap-pairs (l)
  (unless (null l)
    (let ((b (second l)))
      (if (not b) (return-from swap-pairs l))
      (let ((c (second (second l))))
        (format t "~a~%" c)
        (cons (car b) (list (cons (car l) (list (swap-pairs c)))))))))
