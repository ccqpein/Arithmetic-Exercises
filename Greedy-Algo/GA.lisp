(defvar data '((1 2) (2 4) (3 4) (3 6) (3 5) (4 7) (4 5) (7 8) (6 9)))

(defun GA (data)
  (let ((sortedData (sort data #'< :key #'cadr)))
    (do* ((tail sortedData (cdr tail))
          (this (car sortedData) (car tail))
          (key '(0 0))
          (result))
         ((eql tail nil) result)
      (if (>= (car this) (cadr key))
          (setf key this
                result (append result (list this)))
          ;(print this)
          ))))
