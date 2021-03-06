(defun countSmaller (nums)
  (let ((renum '()))
    (do* ((nnums nums (cdr nnums))
          (num (car nnums)
               (car nnums))
          (numl (cdr nnums)
                (cdr nnums)))
         ((eql nil numl) (setf renum (append renum (list 0))))
      ((lambda (x nums)
         (let ((count 0))
           (dolist (y nums count)
             (if (< y x) (incf count)))
           (setf renum (append renum (list count))))) num numl))
    renum))
