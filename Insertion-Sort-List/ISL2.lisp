(defvar *test '(2 1 3))

(defun insert-sort-list (l)
  (do* ((restL (cdr l) (cdr restL))
        (reL '())
        (thisN (car l) (car restL)))
       ((eql 'nil restL) reL)
    ))
