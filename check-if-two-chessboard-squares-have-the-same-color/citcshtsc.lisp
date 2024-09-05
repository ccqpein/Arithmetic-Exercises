(defun check-two-chessboards (co1 co2)
  (eq (apply #'eq (mapcar #'evenp (mapcar #'char-code (concatenate 'list co1))))
      (apply #'eq (mapcar #'evenp (mapcar #'char-code (concatenate 'list co2)))))
  )

(assert (check-two-chessboards "a1" "c3"))
(assert (not (check-two-chessboards "a1" "h3")))
