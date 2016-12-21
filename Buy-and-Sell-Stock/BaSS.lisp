(defvar *test1 '())
(defvar *test2 '(2 1 2 1 0 1 2))
(defvar *test3 '(1))
(defvar *test4 '(7 1 5 3 6 4))
(defvar *test5 '(2 4 1))

(defun max-profit (prices)
  (do* ((max 0)
        (min (first prices))
        (result 0 (if (< result 0) (setf result 0) result))
        (restL (cdr prices) (cdr restL))
        (this (car restL) (car restL)))
       ((eql restL '())
        (return-from max-profit (if (numberp min)
                                    result
                                    0)))
    (cond ((< this min)
           (setf min this))
          ((> (- this min) result)
           (setf max this
                 result (- max min))))))
