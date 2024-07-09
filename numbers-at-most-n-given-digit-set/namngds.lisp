(defparameter *tests* `(
                        ((1 2 3 4 5 6 7 8 9)
                         899894860
                         392738517)
                        ((1 3 5 7)
                         99
                         20)
                        ))

(defun num-helper (n)
  (do (result
       (mm 10)
       )
      ((= n (mod n mm))
       (push (mod n mm) result)
       result)
    (push (mod n mm) result)
    (setf n (floor (/ n 10)))
    ))

(defun filter-count (digits start end)
  (loop with all = (loop for i from start to end collect i)
        for n in all
        when (member n digits)
          count 1
        ))

(defun at-most-n-given-digit-set (digits n)
  (let ((n (num-helper n)))
    (labels ((helper (digits n start-with-zero)
               (if (= 1 (length n))
                   (return-from helper (filter-count digits 0 (first n))))

               (+ (* (filter-count digits 1 (1- (first n))) (expt (length digits) (1- (length n))))
                  (if start-with-zero
                      (helper digits (make-list (1- (length n)) :initial-element 9) t)
                      0)
                  (if (member (first n) digits)
                      (helper digits (cdr n) nil)
                      0))
               ))
      (helper digits n t)
      )))

(defun main ()
  (dolist (tt *tests*)
    (assert (= (at-most-n-given-digit-set (first tt) (second tt))
               (third tt)))
    ))
