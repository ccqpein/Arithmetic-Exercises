(defun remove-duplicates- (str)
  (let ((stack '()))
    (do* ((strs (coerce str 'list) (cdr strs))
          (this (car strs) (car strs))
          (last)
          )
         ((eq strs nil) (concatenate 'string stack))

      (if (eq this last)
          (setf stack (butlast stack)
                last (car (last stack)))
          (progn (setf last this
                       stack (append stack (list this))))))))
