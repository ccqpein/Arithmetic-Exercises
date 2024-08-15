(defun lemonade-change (bills)
  (let ((flag (and (= 5 (nth 0 bills))
                   (< (nth 1 bills) 20)))
        (a 0)
        (b 0))
    (loop for x in bills
          unless flag
            do (return-from lemonade-change flag)

          do (cond ((= x 5)
                    (incf a 1))
                   ((= x 10)
                    (setf flag (> a 0)
                          a (1- a)
                          b (1+ b)))
                   ((> b 0)
                    (setf flag (> a 0)
                          a (1- a)
                          b (1- b)))
                   (t (setf flag (> a 2)
                            a (- a 3)))))
    flag
    ))

(lemonade-change '(5 5 5 10 20))
(lemonade-change '(5 5 10 10 20))

