(defun max-non-overlapping (l target)
  (let ((memo (append (maplist (lambda (l) (apply #'+ l)) l) '(0))))
    (loop with bucket = '()
          for v in memo
          for need = (+ v target)
          when (member need bucket)
            count 1
            and do (setf bucket '())
          do (push v bucket))))
