(defun judge-circle (moves)
  (loop with h = 0
        and c = 0
        for cc in (concatenate 'list moves)
        do (cond ((string= cc #\U) (incf h 1))
                 ((string= cc #\D) (decf h 1))
                 ((string= cc #\L) (decf c 1))
                 ((string= cc #\R) (incf c 1)))
        finally (return (and (zerop c) (zerop h)))))

(defun main ()
  (assert (judge-circle "UD"))
  (assert (not (judge-circle "LL"))))
