(defun largest-integer (num)
  (let (cache)
    (setf cache
          (reverse
           (loop with nn = num
                 while (/= nn 0)
                 collect (mod nn 10) into aa
                 do (setf nn (floor (/ nn 10)))
                 finally (return aa))))

    (let ((re (helper cache 0)))
      (loop for ind from 0 below (length cache)
            sum (* (nth ind re)
                   (expt 10 (- (length cache) ind 1)))))))

(defun helper (nums ind)
  (if (= ind (length nums)) (return-from helper nums))

  (let (maxx)
    (loop with max = ind
          for d from ind below (length nums)
          if (and (= (mod (nth d nums) 2)
                     (mod (nth ind nums) 2))
                  (> (nth d nums) (nth max nums)))
            do (setf max d)
          finally (setf maxx max))

    (let ((a (nth maxx nums)))
      (setf (nth maxx nums) (nth ind nums)
            (nth ind nums) a)
      (helper nums (1+ ind)))))

(defun main ()
  (assert (= 3412 (largest-integer 1234)))
  (assert (= 87655 (largest-integer 65875))))
