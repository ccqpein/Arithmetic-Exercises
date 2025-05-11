(defun generate (num-rows)
  (labels ((make-new (row)
             (cond ((= 0 (length row))
                    '(1))
                   ((= 1 (length row))
                    '(1 1))
                   (t (append
                       '(1)
                       (loop for (x y) on row by #'cdr
                             while y
                             collect (+ x y))
                       '(1))))))
    (loop with a = '()
          for n from 1 to num-rows
          do (setf a (make-new a))
          collect a)))

(defun main ())
