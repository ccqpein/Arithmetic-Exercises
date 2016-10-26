(defun split (s)
  (mapcar #'digit-char-p (coerce s 'list)))

(defun isHappy (n)
  (cond ((< n 1) "wrong")
        ((= n 1) 1)
        (t
         (isHappy (apply #'+
                         (mapcar #'(lambda (x) (* x x))
                                 (split (write-to-string n))))))
        ))
