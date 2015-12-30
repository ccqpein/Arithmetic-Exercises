(defun make-squares-list (n)
  (let ((squareList))
    (setf squareList (mapcar #'(lambda (x)
                                 (* x x))
                             (loop for i from 0 to n collect i)))
    squareList))


