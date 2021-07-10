(defun ship-within-days (weights days)
  (let ((left (apply #'max weights))
        (right (apply #'+ weights)))
    (loop
      while (< left right)
      for mid = (floor (/ (+ left right) 2))
      and need = 1
      and cur = 0
      ;;do (format t "mid: ~d, need: ~d, cur: ~d~%" mid need cur)
      do (loop
           for v in weights
           if (> (+ cur v) mid)
             do (incf need)
             and do (setf cur 0)
           end
           do (incf cur v)
           )
      ;;do (format t "need: ~d, cur: ~d~%" need cur)
      if (> need days)
        do (setf left (1+ mid))
      else
        do (setf right mid)
      end 
      finally (return left)
      )))

(defun main ()
  (assert (= 15 (ship-within-days '(1 2 3 4 5 6 7 8 9 10) 5)))
  (assert (= 6 (ship-within-days '(3 2 2 4 1 4) 3)))
  (assert (= 3 (ship-within-days '(1 2 3 1 1) 4)))
  )
