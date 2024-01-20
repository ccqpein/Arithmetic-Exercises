(defun number-to-list (n)
  (loop
    with base = 10
    collect (mod n base) into result
    if (< n 10)
      return (reverse result)
    else
      do (setf n (floor (/ n base)))
    ))

(defun list-number-join (nl)
  (let ((ex (expt 10 (1- (length nl)))))
    (loop for n in nl
          sum (* ex n) into result
          do (setf ex (/ ex 10))
          finally (return result)
          )))

(defun min-max-difference (n)
  (let* ((a (number-to-list n))
         (pair (cons 0 (car a))))

    (loop for x in a
          if (/= x 9)
            do (setf (car pair) x)
            and return nil)

    (multiple-value-bind (largest smallest)
        (loop for x in a
              collect (if (= x (car pair)) 9 x) into largest
              collect (if (= x (cdr pair)) 0 x) into smallest
              finally (return (values largest smallest)))
      (- (list-number-join largest)
         (list-number-join smallest)))
    ))

(defun main ()
  (format t "~a~%" (min-max-difference 11891))
  (format t "~a~%" (min-max-difference 90)))
