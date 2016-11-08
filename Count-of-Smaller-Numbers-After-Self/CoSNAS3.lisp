(defvar test '(5 2 6 1))

(defun small-right (l)
  (do* ((a (car l) (car b))
        (b (cdr l) (cdr b))
        (reL '()))
       ((eql b '()) (return-from small-right (append reL '(0))))
    (let ((nn 0))
      (loop for i in b
         do (if (< i a)
                (setf nn (1+ nn))))
      (setf reL (append reL (list nn))))
    (print reL)))
