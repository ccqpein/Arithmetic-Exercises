(defun max-ascending-sum (ll)
  (if (null ll) (return-from max-ascending-sum 0))
  (apply #'max
         (mapcar (lambda (l) (apply #'+ l))
                 (seperate-list-if ll #'<))))

(defun seperate-list-if (ll predict)
  (do ((result '())
       (this (first ll))
       (bucket (list (first ll)))
       (rest (cdr ll) (cdr rest)))
      ((null rest)
       (if bucket (push bucket result))
       (reverse result))

    ;;(format t "~a ~a ~a~%" this bucket rest)
    (if (funcall predict (first rest) this)
        (progn
          (push bucket result)
          (setf this (car rest)
                bucket (list this)))
        (progn 
          (setf this (car rest)
                bucket (append bucket (list this)))))
    ))

(defun main ()
  (assert (= (max-ascending-sum '(10 20 30 5 10 50)) 65))
  (assert (= (max-ascending-sum '(12 17 15 13 10 11 12)) 33))
  (assert (= (max-ascending-sum '(10 20 30 40 50)) 150)))
