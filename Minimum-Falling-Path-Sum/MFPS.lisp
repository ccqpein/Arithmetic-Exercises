(defun min-falling-path-sum (ll)
  (do* ((len (length ll))
        (temp (car ll))
        (left (cdr ll) (cdr left))
        )
       ((not left) (reduce #'min temp))
    (setf temp
          (loop
             with cache = (car left)
             for ind from 0 to (1- len)
             collect (+ (nth ind cache)
                        (min (nth (max (1- ind) 0) temp)
                             (nth (min (1+ ind) (1- len)) temp)
                             (nth ind temp)
                             ))
               ))))

(defun main ()
  (min-falling-path-sum '((1 2 3) (4 5 6) (7 8 9))))
