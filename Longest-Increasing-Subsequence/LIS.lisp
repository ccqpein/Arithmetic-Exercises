(defun length-of-lis (nums)
  (let ((cache (make-array 2500 :initial-element 0))
        )
    (labels ((helper (nums ind cache)
               (if (/= 0 (aref cache ind))
                   (aref cache ind)
                   (setf (aref cache ind)
                         (loop
                           for i from (1+ ind) below (length nums)
                           if (> (nth i nums) (nth ind nums))
                             collect (1+ (helper nums i cache)) into aa
                           finally (return (handler-case (apply #'max aa)
                                             (error () 1))))))))
      (loop
        for ind from (1- (length nums)) downto 0
        maximize (helper nums ind cache)))))

(defun main ()
  (assert (= (length-of-lis '(10 9 2 5 3 7 101 18)) 4))
  (assert (= (length-of-lis '(0 1 0 3 2 3)) 4))
  (assert (= (length-of-lis '(7 7 7 7 7 7 7)) 1))
  (assert (= (length-of-lis '(1 3 6 7 9 4 10 5 6)) 6)))
