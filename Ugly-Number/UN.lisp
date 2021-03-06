(defun isUgly (num)
  (cond ((member num '(1 2 3 5)) (return-from isUgly 'T))
        ((= num 0) (return-from isUgly nil))
        ((= (mod num 2) 0) (progn (setf num (/ num 2)) (isUgly num)))
        ((= (mod num 3) 0) (progn (setf num (/ num 3)) (isUgly num)))
        ((= (mod num 5) 0) (progn (setf num (/ num 5)) (isUgly num)))
        (t (return-from isUgly nil))))
