;; alexandria is very good toolbox
(ql:quickload "alexandria")

(defun can-arrange (arr k)
  (let ((table (make-hash-table)))
    (loop
      for num in arr
      for m = (mod num k)
      and cm = (mod (- num) k)
      for cm-v = (multiple-value-list (gethash cm table))
      if (cadr cm-v) ;; has cm
        do (decf (gethash cm table))
        and do (if (= 1 (car cm-v))
                   (setf (gethash cm table) -1))
      else
        do (if (cadr (multiple-value-list (gethash m table)))
               (incf (gethash m table))
               (setf (gethash m table) 1))
      end
      finally (return (every #'(lambda (v) (= -1 v)) (alexandria:hash-table-values table)))
      )))

(assert (can-arrange '(1 2 3 4 5 10 6 7 8 9) 5))
(assert (can-arrange '(1 2 3 4 5 6) 7))
(assert (not (can-arrange '(1 2 3 4 5 6) 10)))
