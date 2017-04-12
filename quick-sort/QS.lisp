(defun quick-sort (l)
  (let ((key (car l)))
    (if (eql nil l)
        '()
        (append (quick-sort
                 (loop for i in (cdr l) when (< i key)
                    collect i))
                (list key)
                (quick-sort
                 (loop for i in (cdr l) when (>= i key)
                    collect i))))))
